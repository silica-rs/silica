use collections::string::String;
pub mod serial;

pub trait GetByName<T> {
    fn get_by_name(name: &String) -> T;
}
