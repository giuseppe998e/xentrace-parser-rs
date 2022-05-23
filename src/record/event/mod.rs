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

#[cfg(test)]
mod tests {
    use super::{Event, EventCode, EVENT_EXTRA_MAXLEN};

    #[test]
    fn equality_test() {
        let event1 = Event {
            code: EventCode::from_u32(0x00015003),
            tsc: 0,
            extra: [None; EVENT_EXTRA_MAXLEN],
        };
        let event2 = Event {
            code: EventCode::from_u32(0x00015003),
            tsc: 0,
            extra: [None; EVENT_EXTRA_MAXLEN],
        };

        assert_eq!(event1, event2);
    }

    #[test]
    fn ordinariness_test() {
        let mut events = vec![
            Event {
                code: EventCode::from_u32(0x00015004),
                tsc: 4,
                extra: [None; EVENT_EXTRA_MAXLEN],
            },
            Event {
                code: EventCode::from_u32(0x00015001),
                tsc: 0,
                extra: [None; EVENT_EXTRA_MAXLEN],
            },
            Event {
                code: EventCode::from_u32(0x00015003),
                tsc: 98,
                extra: [None; EVENT_EXTRA_MAXLEN],
            },
        ];

        events.sort_unstable();

        let first_event = Event {
            code: EventCode::from_u32(0x00015001),
            tsc: 0,
            extra: [None; EVENT_EXTRA_MAXLEN],
        };
        let last_event = &events[2];

        assert_eq!(last_event.code.into_u32(), 0x00015003);
        assert_eq!(first_event.code.into_u32(), 0x00015001);
    }
}
