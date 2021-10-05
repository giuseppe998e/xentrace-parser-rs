use std::{
    cmp::Ordering,
    convert::TryInto,
    fmt::{Debug, Formatter, Result},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EventCode {
    code: u32,
    main: u16,
    sub: u8,
    minor: u16,
}

impl EventCode {
    pub fn from_u32(code: u32) -> Self {
        Self {
            code,
            main: ((code & 0x0FFF0000) >> 16).try_into().unwrap(),
            sub: ((code & 0x0000F000) >> 12).try_into().unwrap(),
            minor: (code & 0x00000FFF).try_into().unwrap(),
        }
    }

    pub fn into_u32(&self) -> u32 {
        self.code
    }

    pub fn get_main(&self) -> u16 {
        self.main
    }

    pub fn get_sub(&self) -> u8 {
        self.sub
    }

    pub fn get_minor(&self) -> u16 {
        self.minor
    }
}

impl Debug for EventCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "EventCode {{ code: {:#010X}, main: {:#06X}, sub: {:#03X}, minor: {:#05X} }}",
            self.code, self.main, self.sub, self.minor
        )
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Event {
    code: EventCode,
    tsc: u64,
    extra: Vec<u32>,
}

impl Event {
    // CRATE FNs
    pub(crate) fn new(code: u32, tsc: u64, extra: Vec<u32>) -> Self {
        Self {
            code: EventCode::from_u32(code),
            tsc,
            extra,
        }
    }

    // PUBLIC FNs
    pub fn get_code(&self) -> EventCode {
        self.code
    }

    pub fn get_extra(&self) -> &[u32] {
        self.extra.as_slice()
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
    use crate::TRC_TRACE_CPU_CHANGE;

    #[test]
    fn event_new() {
        let event = Event::new(TRC_TRACE_CPU_CHANGE, 0, vec![]);
        assert_eq!(event, Event::new(TRC_TRACE_CPU_CHANGE, 0, vec![]));
    }

    #[test]
    fn event_tsc() {
        let event = Event::new(TRC_TRACE_CPU_CHANGE, 1382371621213, vec![]);
        assert_eq!(event.get_tsc(), 1382371621213);
    }

    #[test]
    fn event_extra() {
        let event = Event::new(TRC_TRACE_CPU_CHANGE, 0, vec![1, 3, 5]);
        assert_eq!(event.get_extra().len(), 3);
        assert_eq!(event.get_extra()[2], 5);
    }
}
