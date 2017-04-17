use collections::string::String;
pub mod serial;
pub mod gpio;

pub trait GetByName<T> {
    fn get_by_name(name: &String) -> T;
}
