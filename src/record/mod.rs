mod domain;
pub use domain::*;

mod event;
pub use event::Event;

use std::cmp::Ordering;

#[derive(Clone, Eq, Debug)]
pub struct Record {
    cpu: u8,
    domain: Domain,
    event: Event,
}

impl Record {
    // CRATE FNs
    pub(crate) fn new(cpu: u8, domain: Domain, event: Event) -> Self {
        Self { cpu, domain, event }
    }

    // PUBLIC FNs
    pub fn get_cpu(&self) -> u8 {
        self.cpu
    }

    pub fn get_domain(&self) -> Domain {
        self.domain
    }

    pub fn get_event(&self) -> Event {
        self.event.clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record() {
        let domain = Domain::from_u32(0x00018000);
        let event = Event::new(0x0001f003, 0, vec![]);
        let record = Record::new(5, domain, event);

        assert_eq!(record.get_cpu(), 5);
        assert_eq!(record.get_domain(), domain);
        assert_eq!(record.get_event(), Event::new(0x0001f003, 0, vec![]));
    }
}
