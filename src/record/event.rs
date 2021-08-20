use std::cmp::Ordering;

#[derive(Clone, Eq, Debug)]
pub struct Event {
    code: u32,
    tsc: u64,
    extra: Vec<u32>,
}

impl Event {
    // CRATE FNs
    pub(crate) fn new(code: u32, tsc: u64, extra: Vec<u32>) -> Self {
        Self { code, tsc, extra }
    }

    // PUBLIC FNs
    pub fn get_code(&self) -> u32 {
        self.code
    }

    pub fn get_extra(&self) -> &Vec<u32> {
        &self.extra
    }

    pub fn get_tsc(&self) -> u64 {
        self.tsc
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.tsc == other.tsc
            && self
                .extra
                .iter()
                .zip(other.extra.iter())
                .all(|(a, b)| a == b)
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

#[cfg(test)]
mod tests {
    use super::*;

    const EVENT_CODE: u32 = 0x0001f003;

    #[test]
    fn event_new() {
        let event = Event::new(EVENT_CODE, 0, vec![]);
        assert_eq!(event, Event::new(EVENT_CODE, 0, vec![]));
    }

    #[test]
    fn event_tsc() {
        let event = Event::new(EVENT_CODE, 1382371621213, vec![]);
        assert_eq!(event.get_tsc(), 1382371621213);
    }

    #[test]
    fn event_extra() {
        let event = Event::new(EVENT_CODE, 0, vec![1, 3, 5]);
        assert_eq!(event.get_extra().len(), 3);
        assert_eq!(event.get_extra()[2], 5);
    }
}
