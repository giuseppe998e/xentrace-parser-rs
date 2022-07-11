use std::fmt::{Debug, Formatter, Result};

/// Contains the event code read as a 32-bit unsigned big-endian integer.
#[derive(Clone, Copy, Eq)]
pub struct EventCode {
    code: u32,
    /// Class, can be used to filter events.
    pub(crate) main: u16,
    /// Subclass, can also be used to filter events.
    pub(crate) sub: u8,
    /// Minor, identifies the event in its class and subclass.
    pub(crate) minor: u16,
}

impl EventCode {
    /// Returns the event class, can be used to filter events.
    pub fn main(&self) -> u16 {
        self.main
    }

    /// Returns the event subclass, can also be used to filter events.
    pub fn sub(&self) -> u8 {
        self.sub
    }

    /// Returns the event minor, identifies the event in its class and subclass.
    pub fn minor(&self) -> u16 {
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

impl PartialEq for EventCode {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl From<u32> for EventCode {
    fn from(val: u32) -> Self {
        Self {
            code: val,
            main: ((val & 0x0FFF0000) >> 16) as u16,
            sub: ((val & 0x0000F000) >> 12) as u8,
            minor: (val & 0x00000FFF) as u16,
        }
    }
}

impl From<EventCode> for u32 {
    fn from(val: EventCode) -> Self {
        val.code
    }
}

impl From<EventCode> for u64 {
    fn from(val: EventCode) -> Self {
        u64::from(u32::from(val))
    }
}

impl From<EventCode> for i64 {
    fn from(val: EventCode) -> Self {
        i64::from(u32::from(val))
    }
}

impl From<EventCode> for u128 {
    fn from(val: EventCode) -> Self {
        u128::from(u32::from(val))
    }
}

impl From<EventCode> for i128 {
    fn from(val: EventCode) -> Self {
        i128::from(u32::from(val))
    }
}

impl TryFrom<EventCode> for usize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: EventCode) -> std::result::Result<Self, Self::Error> {
        usize::try_from(u32::from(value))
    }
}

impl TryFrom<EventCode> for isize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: EventCode) -> std::result::Result<Self, Self::Error> {
        isize::try_from(u32::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::EventCode;

    #[test]
    fn equality_test() {
        let ecode1 = EventCode::from(0x00015003);
        let ecode2 = EventCode::from(0x00015003);

        assert_eq!(ecode1, ecode2);
        assert_eq!(ecode1.main, ecode2.main);
        assert_eq!(ecode1.sub, ecode2.sub);
        assert_eq!(ecode1.minor, ecode2.minor);
    }

    #[test]
    fn not_full_equality_test() {
        let ecode1 = EventCode::from(0x00015003);
        let ecode2 = EventCode::from(0x01015003);

        assert_ne!(ecode1, ecode2);
        assert_ne!(ecode1.main, ecode2.main);

        assert_eq!(ecode1.sub, ecode2.sub);
        assert_eq!(ecode1.minor, ecode2.minor);
    }
}
