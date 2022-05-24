mod domain;
pub use domain::{Domain, DomainType};

mod event;
pub use event::{Event, EventCode, EVENT_EXTRA_MAXLEN};

use std::cmp::Ordering;

/// Contains information from a single record of the parsed XenTrace binary file.
#[derive(Clone, Eq, Debug)]
pub struct Record {
    /// The processor number (of the host) on which the [`Event`](event::Event) occurred.
    pub cpu: u16,
    /// The [`Domain`](domain::Domain) on which the [`Event`](event::Event) occurred.
    pub domain: Domain,
    /// The information of the [`Event`](event::Event) of this record.
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
