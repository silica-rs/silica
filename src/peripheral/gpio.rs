
pub trait Input : Drop {
    fn read(&self) -> bool;
}
pub trait Output : Drop {
    fn get_command(&self) -> bool;
    fn write(&mut self, bool) -> bool;
}
