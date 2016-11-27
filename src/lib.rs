#![feature(needs_panic_runtime)]
#![feature(optin_builtin_traits)]
#![feature(collections)]
#![feature(alloc)]
#![feature(const_fn)]
#![feature(unboxed_closures)]

#![no_std]

#[cfg(not(test))]
extern crate silica_panic;
extern crate silica_allocator as allocator;
extern crate alloc as liballoc;
extern crate collections;

pub mod alloc {
    pub use allocator::init;
}

pub mod thread;
pub mod sync;

pub mod time;
pub mod io;

pub mod peripheral;
