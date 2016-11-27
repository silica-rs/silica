use core::ptr;
use time::{Instant, Duration};
use liballoc::arc::Arc;
use liballoc::boxed::Box;
use liballoc::raw_vec::RawVec;

use sync::InterruptSafe;

const PRIORITY_MAX: usize = 5;
// Option<Shared<Thread>> : Shared is !Sync !Send which prevent from being static
static mut READY_LIST: [*const InterruptSafe<ThreadInner>; PRIORITY_MAX] = [ptr::null_mut(); PRIORITY_MAX];

extern {
    fn setup_stack(stack: *const usize, size: usize, f: extern fn(*mut u8), th: *mut u8) -> *const usize;
}

///
/// Thread :
/// - heap allocated
/// - reference counted
///
pub enum ThreadPriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest
}

enum CurrentList {
    None,
    ReadyList,
    DelayedList,
    Blocked
}

struct ThreadInner {
    top_of_stack: *const usize,
    name: &'static str,
    pri: ThreadPriority,
    f: Box<Fn()>,
    current_list: CurrentList,
    next: *const InterruptSafe<ThreadInner>,
    wake_up_time: Instant,
    stack: RawVec<usize>
}
impl ThreadInner {
    pub fn pri_to_idx(&self) -> usize {
        match self.pri {
            ThreadPriority::Lowest => 0,
            ThreadPriority::Low => 1,
            ThreadPriority::Medium => 2,
            ThreadPriority::High => 3,
            ThreadPriority::Highest => 4
        }
    }
}

extern fn wrapper(p: *mut u8) {
    let b: Box<_> = unsafe { Box::from_raw(p as *mut Thread) };
    let inner = (*b).cell.inner();
    (*inner.f)();
}

pub struct Thread {
    cell: Arc<InterruptSafe<ThreadInner>>
}
impl Thread {
    pub fn new(f: Box<Fn()>, name: &'static str,  stack_size: usize, pri: ThreadPriority) -> Thread {
        Thread { cell: Arc::new(InterruptSafe::new(ThreadInner {
                name: name,
                top_of_stack: ptr::null(),
                pri: pri,
                current_list: CurrentList::None,
                f: f,
                next: ptr::null_mut(),
                wake_up_time: Instant::new(),
                stack: RawVec::with_capacity(stack_size)
            }))
        }
    }

    pub fn name(&self) -> &'static str {
        self.cell.inner().name
    }

    /// this takes CriticalSection to ensure it runs in a safe block IT
    fn add_to_end(&mut self) {
        let inner: &mut ThreadInner = &mut *self.cell.inner();
        let ptr = match inner.current_list {
            CurrentList::None => return,
            CurrentList::ReadyList => {
                unsafe {
                    &mut READY_LIST[inner.pri_to_idx()]
                }
            }
            _ => panic!("woops")
        };

        unsafe {
            match (*ptr).as_ref() {
                None => { *ptr = &*self.cell }
                Some(mut head) => {
                    while !(*head.inner()).next.is_null() {
                        head = (*head.inner()).next.as_ref().unwrap();
                    }
                    head.inner().next = &*self.cell
                }
            }
        };
    }

    pub fn start(&mut self) -> &mut Thread {
        let inner = &mut *self.cell.inner();
        match inner.current_list {
            CurrentList::None => return self,
            _ => {}
        }

        let t = Box::new(self.clone());
        let pt = Box::into_raw(t);

        unsafe {
            inner.top_of_stack = setup_stack(inner.stack.ptr(), inner.stack.cap(), wrapper, pt as *mut u8);
        }
        // push this thread to the ready list
        self.add_to_end();
        // yield
        self
    }

    pub fn join(&self, delay: Duration) -> bool {
        false
    }
}

unsafe impl Sync for Thread {}
impl Clone for Thread {
    fn clone(&self) -> Thread {
        Thread { cell: self.cell.clone() }
    }
}
impl Drop for Thread {
    fn drop(&mut self) {

    }
}

fn StartScheduler() -> ! {
    // create idle task responsible of freeing terminated threads.
    unimplemented!();
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Threadlist_new() {

    }
}
