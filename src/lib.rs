#![feature(needs_panic_runtime)]
#![feature(optin_builtin_traits)]
#![feature(alloc)]
#![feature(const_fn)]
#![feature(unboxed_closures)]

#![no_std]

//extern crate core;
extern crate alloc;
#[cfg(not(test))]
extern crate silica_core_allocator;
pub extern crate silica_core_sync as sync;

pub mod time;
// pub mod io;

// pub mod peripheral;
