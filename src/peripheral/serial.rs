use collections::string::String;
use io::{Read, Write, Receive};

pub enum BitCount {
    SevenBits,
    EightBits
}

pub enum Parity {
    None,
    Even,
    Odd,
    Mark,
    Space
}

pub enum StopBit {
    OneBit,
    OneDotFiveBit,
    TwoBits
}

pub trait Serial : Read + Write + Receive + Drop {
    fn setup(&mut self, baudrate:usize, word_len: BitCount, parity: Parity, stop_bit: StopBit) -> Result<(), String>;
    fn baudrate(&self) -> usize;
    fn open(&mut self) -> Result<(), String>;
    fn close(&mut self);
}
