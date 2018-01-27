
pub trait Input {
    fn read(&self) -> bool;
}
pub trait Output {
    fn get_command(&self) -> bool;
    fn write(&mut self, bool) -> bool;
}
