// timestamp = 41 bits
// server id = 5 bits
// worker id = 5 bits
// node counter =  = 12 bits
pub struct Constructor {
    epoch: u64,
    // 5 bits
    server: u8,
    // 5 bits
    worker: u8,
    // 12 bits
    counter: u16,
}

impl Constructor {
    const TIMESTAMP_BITS: u8 = 41;
    const SERVER_BITS: u8 = 5;
    const WORKER_BITS: u8 = 5;
    const COUNTER_BITS: u8 = 12;

    const MAX_SERVER: usize = (-1_isize ^ (-1_isize << Self::TIMESTAMP_BITS)) as usize; // 2^5-1
    const MAX_WORKER: usize = (-1_isize ^ (-1_isize << Self::SERVER_BITS)) as usize; // 2^5-1
    const MAX_COUNTER: usize = (-1_isize ^ (-1_isize << Self::COUNTER_BITS)) as usize; // 2^12-1

    const TIMESTAMP_SHIFT: u8 = Self::COUNTER_BITS + Self::SERVER_BITS + Self::WORKER_BITS;
    const SERVER_SHIFT: u8 = Self::COUNTER_BITS + Self::WORKER_BITS;
    const WORKER_SHIFT: u8 = Self::COUNTER_BITS;

    pub fn get(&self) -> u64 {
        todo!()
    }
}
