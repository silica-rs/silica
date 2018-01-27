use alloc::string::String;
pub mod serial;
pub mod gpio;

pub trait GetByName<T> {
    fn get_by_name(name: &String) -> T;
}

/// Peripheral trait
/// Peripheral may also implement Drop to handle the Deinit but that's not mandatory.
pub trait Peripheral {
    fn init(&self) -> Result<(), String>;
}
