#![no_std]

#[cfg(any(
    target = "thumbv6m-none-eabi",
    target = "thumbv7m-none-eabi",
    target = "thumbv7em-none-eabi",
    target = "thumbv7em-none-eabihf",
))]
/// Locks based on the `cortex_m::interrupt::free` interruption masking. They may not be good for multi-core purposes, but should be fine for mono-cores.
pub mod cortex_m_interrupt_free;

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
/// This module picks the "best" locks available for your target.
///
/// "Best" is defined as "safety first, performance second": a better lock is a lock that
/// ensures memory safety in a more "exhaustive" way, before being one that uses less CPU instructions.
pub mod recommended {
    use crate::cortex_m_interrupt_free::{NonReentrantMutex, ReentrantMutex};
}
