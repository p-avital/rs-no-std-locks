#![no_std]

mod private_traits {
    pub trait Unlockable {
        fn unlock(&self);
    }
    pub trait HasContent {
        type ContentType;
        fn get_content(&self) -> *mut Self::ContentType;
    }
}

use private_traits::{HasContent, Unlockable};

pub trait Lockable<Params>
where
    Self: Unlockable + Sized,
{
    fn try_lock<'l>(&'l self, params: &Params) -> Option<Guard<'l, Self>>;
}

pub struct Guard<'l, Lock: Unlockable> {
    parent: &'l Lock,
}

impl<'l, Lock: Unlockable> Drop for Guard<'l, Lock> {
    fn drop(&mut self) {
        self.parent.unlock();
    }
}

impl<'l, Lock: HasContent + Unlockable> core::ops::Deref for Guard<'l, Lock> {
    type Target = <Lock as HasContent>::ContentType;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.parent.get_content() }
    }
}

impl<'l, Lock: HasContent + Unlockable> core::ops::DerefMut for Guard<'l, Lock> {
    fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
        unsafe { &mut *self.parent.get_content() }
    }
}

#[cfg(any(
    target = "thumbv6m-none-eabi",
    target = "thumbv7m-none-eabi",
    target = "thumbv7em-none-eabi",
    target = "thumbv7em-none-eabihf",
))]
pub mod cortex_m_interrupt_free;

#[cfg(any(
    target = "thumbv6m-none-eabi",
    target = "thumbv7m-none-eabi",
    target = "thumbv7em-none-eabi",
    target = "thumbv7em-none-eabihf",
))]
pub mod recommended {
    use crate::cortex_m_interrupt_free::{NonReentrantMutex, ReentrantMutex};
}
