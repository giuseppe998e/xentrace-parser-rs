mod ecode;
pub use ecode::EventCode;

use std::cmp::Ordering;

pub const EVENT_EXTRA_MAXLEN: usize = 7;

#[derive(Debug, Clone, Eq)]
pub struct Event {
    pub code: EventCode,
    pub tsc: u64,
    pub extra: [Option<u32>; EVENT_EXTRA_MAXLEN],
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.tsc == other.tsc && self.extra == other.extra
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tsc.cmp(&other.tsc)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
