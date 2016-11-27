use collections::string::String;
use sync::mpsc::Sender;
use core::{ptr, cmp};
use liballoc::raw_vec::RawVec;

#[derive(Debug)]
pub struct Error {
    pub msg: String
}

pub trait Read {
    fn read(&mut self, dest: &mut [u8]) -> Result<usize, Error>;
}

pub trait Receive {
    fn on_recv(&mut self, s: Sender<()>);
}

pub trait Write {
    fn write(&mut self, dest: &[u8]) -> Result<usize, Error>;
}

pub enum SeekFrom {
    Start(u64),
    End(u64),
    Current(i64)
}

pub trait Seek {
    fn size(&self) -> u64;
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error>;
}

pub trait Connect {
    fn connect() -> Result<(), Error>;
    fn disconnect() -> Result<(), Error>;
}


pub struct Fifo<T> {
    v: RawVec<T>,   // vector
    cap: usize,     // for ZSTs RawVec says cap is usize::MAX which is not always wanted. this will basically act as a saturable counter.
    w: usize,       // next idx to write to
    r: Option<usize>  // idx to read from if any
}
impl<T> Fifo<T> {
    pub fn new(capacity: usize) -> Fifo<T> {
        let cap = cmp::max(1, capacity);
        Fifo { v: RawVec::with_capacity(cap), cap: cap, w: 0, r: None }
    }

    pub fn push_front(&mut self, item: T) -> Result<(), T> {
        if let Some(r) = self.r {
            if self.w == r {
                return Err(item)
            }
        }

        unsafe { ptr::write(self.v.ptr().offset(self.w as isize), item); }

        if let None = self.r {
            self.r = Some(self.w);
        }

        self.w += 1;
        // more efficient that %
        if self.w == self.cap {
            self.w = 0;
        }

        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut r = match self.r {
            None => return None,
            Some(r) => r
        };
        let item = unsafe { ptr::read(self.v.ptr().offset(r as isize)) };
        r += 1;

        if r == self.w {
            self.r = None;
        } else {
            if r == self.cap {
                r = 0;
            }
            self.r = Some(r);
        }
        Some(item)
    }
}

#[cfg(test)]
mod test {
    use super::Fifo;

    fn helper_setup_offset() -> Fifo<u32> {
        let mut v: Fifo<u32> = Fifo::new(10);
        for i in 0..5 {
            assert_eq!(v.push_front(i), Ok(()));
        }
        for i in 0..5 {
            assert_eq!(v.pop_back().unwrap(), i);
        }
        v
    }

    #[test]
    fn fifo_basic_ops() {
        let mut v: Fifo<u32> = Fifo::new(10);

        // test basic push/pop
        for i in 0..5 {
            assert_eq!(v.push_front(i), Ok(()));
        }
        for i in 0..5 {
            assert_eq!(v.pop_back().unwrap(), i);
        }
    }

    #[test]
    fn fifo_wrapping() {
        let mut v = helper_setup_offset();
        // test wrapping
        for i in 0..10 {
            assert_eq!(v.push_front(i), Ok(()));
        }
        for i in 0..10 {
            assert_eq!(v.pop_back().unwrap(), i);
        }
    }
    #[test]
    fn fifo_overrun() {
        let mut v = helper_setup_offset();
        // test overrun
        for i in 0..10 {
            assert_eq!(v.push_front(i), Ok(()));
        }
        assert_eq!(v.push_front(11), Err(11));
    }

    #[test]
    fn fifo_underrun() {
        let mut v = helper_setup_offset();
        assert_eq!(v.push_front(42), Ok(()));
        assert_eq!(v.pop_back().unwrap(), 42);

        // test underrun
        assert_eq!(v.pop_back(), None);
    }
}
