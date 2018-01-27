use core::cell::UnsafeCell;
use alloc::arc::Arc;
use io::Fifo;
use time::Instant;

#[allow(dead_code)]
pub struct ISRSender {
    inner: Arc<UnsafeCell<Fifo<()>>>
}
impl ISRSender {
    pub fn send(&self) -> Result<(), &'static str> {
        Err("unimplemented!")
    }
}
impl From<Sender<()>> for ISRSender {
    fn from(s: Sender<()>) -> ISRSender {
        ISRSender { inner: s.inner.clone() }
    }
}
impl Drop for ISRSender {
    fn drop(&mut self) {

    }
}

pub struct Sender<T> {
    inner: Arc<UnsafeCell<Fifo<T>>>
}
impl<T> Sender<T> {
    fn new(a: Arc<UnsafeCell<Fifo<T>>>) -> Sender<T> {
        Sender { inner: a }
    }
    pub fn send(&self, e: T) -> Result<(), T> {
        // deref Arc to UnsafeCell
        // get() on UnsafeCell
        // dereference *mut Fifo<T>
        unsafe { (*(*self.inner).get()).push_front(e) }
    }
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        Sender { inner: self.inner.clone() }
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {

    }
}

pub struct Receiver<T> {
    inner: Arc<UnsafeCell<Fifo<T>>>
}
impl<T> Receiver<T> {
    fn new(a: Arc<UnsafeCell<Fifo<T>>>) -> Receiver<T> {
        Receiver { inner: a }
    }
    pub fn receive(&self, delay: Option<Instant>) -> Option<T> {
        let _ = delay;
        unsafe { (*(*self.inner).get()).pop_back() }
    }
}

pub fn channel<T>(length: usize) -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(UnsafeCell::new(Fifo::new(length)));
    (Sender::new(a.clone()), Receiver::new(a.clone()))
}
