use std::{borrow::Borrow, cmp::Ordering, rc::Rc};

mod domain;
pub use domain::{Domain, DomainKind};

mod event;
pub use event::{Event, EventCode, EVENT_EXTRA_MAXLEN};

/// Contains information from a single record of the parsed XenTrace binary file.
#[derive(Clone, Eq, Debug)]
pub struct Record {
    /// The processor number (of the host) on which the [`Event`](event::Event) occurred.
    pub(crate) cpu: u16,
    /// The [`Domain`](domain::Domain) on which the [`Event`](event::Event) occurred.
    pub(crate) domain: Rc<Domain>,
    /// The information of the [`Event`](event::Event) of this record.
    pub(crate) event: Event,
}

impl Record {
    pub fn cpu(&self) -> u16 {
        self.cpu
    }

    pub fn domain(&self) -> &Domain {
        self.domain.borrow()
    }

    pub fn event(&self) -> &Event {
        &self.event
    }
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
