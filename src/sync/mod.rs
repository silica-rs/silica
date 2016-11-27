use core::ops::Deref;
use core::ops::DerefMut;
use core::cell::UnsafeCell;

extern {
    fn critical_section_enter();
    fn critical_section_exit();
}

/// While this is alive no interruption nor context switch may occur.
/// Using a droppable
pub struct CriticalSection<T> {
    inner: *mut T
}

impl<T> CriticalSection<T> {
    pub fn new(item: *mut T) -> CriticalSection<T> {
        unsafe { critical_section_enter() };
        CriticalSection { inner: item }
    }
}
impl<T> Deref for CriticalSection<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.as_ref().unwrap() }
    }
}
impl<T> DerefMut for CriticalSection<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.inner.as_mut().unwrap() }
    }
}
impl<T> Drop for CriticalSection<T> {
    fn drop(&mut self) {
        unsafe { critical_section_exit() };
    }
}
impl<T> !Sync for CriticalSection<T> {}
impl<T> !Send for CriticalSection<T> {}

pub struct InterruptSafe<T> {
    inner: UnsafeCell<T>
}
impl<T> InterruptSafe<T> {
    pub fn new(item: T) -> InterruptSafe<T> {
        InterruptSafe { inner: UnsafeCell::new(item) }
    }

    pub fn inner(&self) -> CriticalSection<T> {
        CriticalSection::new( self.inner.get() )
    }
}

pub mod mpsc;
