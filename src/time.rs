static TICK_COUNT: u64 = 0;

#[derive(Copy, Clone)]
pub struct Instant {
    t: u64
}
impl Instant {
    pub const fn new() -> Self {
        Instant { t: 0 }
    }
    pub fn now() -> Self {
        Instant { t: TICK_COUNT }
    }
    pub fn value(&self) -> u64 {
        self.t
    }
}

pub enum Duration {
    MilliSecond(u64),
    Eternity
}
impl From<u64> for Duration {
    fn from(n: u64) -> Self {
        Duration::MilliSecond(n)
    }
}
