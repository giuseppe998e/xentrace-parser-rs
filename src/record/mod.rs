mod domain;
mod event;

use std::cmp::Ordering;

pub use self::{
    domain::{Domain, DomainKind},
    event::{Event, EventCode, EVENT_EXTRA_CAPACITY},
};

/// Contains information from a single record of the parsed XenTrace binary file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    /// The processor id (of the host) on which the [`Event`](event::Event) occurred.
    pub(crate) cpu: u32,
    /// The [`Domain`](domain::Domain) on which the [`Event`](event::Event) occurred.
    pub(crate) domain: Domain,
    /// The information of the [`Event`](event::Event) of this record.
    pub(crate) event: Event,
}

impl Record {
    /// Returns the processor id (of the host) on which the [`Event`](event::Event) occurred.
    pub fn cpu(&self) -> u32 {
        self.cpu
    }

    /// Returns the [`Domain`](domain::Domain) on which the [`Event`](event::Event) occurred.
    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    /// Returns the information of the [`Event`](event::Event) of this record.
    pub fn event(&self) -> &Event {
        &self.event
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
