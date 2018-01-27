use alloc::string::String;
use io::{Read, Write, Receive};

#[derive(PartialEq, Eq)]
pub enum BitCount {
    SevenBits,
    EightBits,
    NineBits
}

#[derive(PartialEq, Eq)]
pub enum Parity {
    None,
    Even,
    Odd,
    Mark,
    Space
}

#[derive(PartialEq, Eq)]
pub enum StopBit {
    OneBit,
    OneAndAHalfBit,
    TwoBits
}

/// async read, synchronous write
pub trait Serial : Read + Write + Receive {
    fn baudrate(&self) -> usize;
    fn open(&mut self, baudrate:usize, word_len: BitCount, parity: Parity, stop_bit: StopBit) -> Result<(), String>;
    fn close(&mut self);
}
