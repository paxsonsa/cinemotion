pub struct TimeSpec {
    tick_count : u64,
    time_ms: u64,
}

impl TimeSpec {

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn time_ms(&self) -> u64 {
        self.time_ms
    }

    pub fn new(tick_count: u64, time_ms: u64) -> Self {
        TimeSpec {
            tick_count,
            time_ms,
        }
    }
}