extern crate core;
extern crate cortex_m;
use crate::private_traits::*;

pub struct ReentrantMutex<Data, Token> {
    content: core::cell::UnsafeCell<Data>,
    locked_by: core::cell::UnsafeCell<Option<Token>>,
    counter: core::cell::UnsafeCell<usize>,
}

/// An interruption masking based  Reentrant Mutex. Reentrance is handled by comparing
/// the token you pass as you try to lock it. An example of such token would be a pthread_t
/// if you are using pthreads. Explicit token passing means that you are responsible for ensuring that you only pass the same token from calls that you know won't cause races.
impl<Data, Token> HasContent for ReentrantMutex<Data, Token> {
    type ContentType = Data;
    fn get_content(&self) -> *mut Data {
        self.content.get()
    }
}

impl<Data, Token> ReentrantMutex<Data, Token>
where
    for<'any> &'any Token: PartialEq,
{
    pub fn new(data: Data) -> Self {
        ReentrantMutex {
            content: core::cell::UnsafeCell::new(data),
            locked_by: core::cell::UnsafeCell::default(),
            counter: core::cell::UnsafeCell::new(0),
        }
    }
    pub fn try_lock<'l>(&'l self, token: Token) -> Option<Guard<'l, Self>> {
        cortex_m::interrupt::free(|_| unsafe {
            let guard = &mut *self.locked_by.get();
            match guard {
                None => {
                    *guard = Some(token);
                    *self.counter.get() = 1;
                    Some(Guard { parent: self })
                }
                Some(locker) if &*locker == &token => {
                    *self.counter.get() += 1;
                    Some(Guard { parent: self })
                }
                Some(_) => None,
            }
        })
    }
}

impl<Data, Token> Unlockable for ReentrantMutex<Data, Token> {
    fn unlock(&self) {
        cortex_m::interrupt::free(|_| {
            let count = self.counter.get();
            unsafe {
                *count -= 1;
                if *count == 0 {
                    *self.locked_by.get() = None;
                }
            }
        })
    }
}

/// An interruption masking based non-reentrant lock. Simpler and faster than its reentrant
/// counterpart, but you'll have to be careful about possible reentries.
pub struct NonReentrantMutex<Data> {
    content: core::cell::UnsafeCell<Data>,
    locked: core::cell::UnsafeCell<bool>,
}

impl<Data> HasContent for NonReentrantMutex<Data> {
    type ContentType = Data;
    fn get_content(&self) -> *mut Data {
        self.content.get()
    }
}

impl<Data> NonReentrantMutex<Data> {
    pub fn new(data: Data) -> Self {
        NonReentrantMutex {
            content: core::cell::UnsafeCell::new(data),
            locked: core::cell::UnsafeCell::default(),
        }
    }
    pub fn try_lock<'l>(&'l self) -> Option<Guard<'l, Self>> {
        cortex_m::interrupt::free(|_| unsafe {
            let guarded = &mut *self.locked.get();
            match guarded {
                false => {
                    *guarded = true;
                    Some(Guard { parent: self })
                }
                true => None,
            }
        })
    }
}

impl<Data> Unlockable for NonReentrantMutex<Data> {
    fn unlock(&self) {
        cortex_m::interrupt::free(|_| unsafe {
            *self.locked.get() = false;
        })
    }
}
