mod domain;
mod event;

pub use domain::{Domain, DomainType};
pub use event::{Event, EventCode, EVENT_EXTRA_MAXLEN};

use std::cmp::Ordering;

#[derive(Clone, Eq, Debug)]
pub struct Record {
    pub cpu: u16,
    pub domain: Domain,
    pub event: Event,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.cpu == other.cpu && self.domain == other.domain && self.event == other.event
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.event.cmp(&other.event)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
