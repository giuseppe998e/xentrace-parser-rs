use std::fmt::{Debug, Formatter, Result};

/// Contains the event code read as a 32-bit unsigned big-endian integer.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EventCode(u32);

impl EventCode {
    /// Returns the complete event code.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Returns the event class, can be used to filter events.
    pub fn main(&self) -> u32 {
        (self.0 & 0x0FFF0000) >> 16
    }

    /// Returns the event subclass, can also be used to filter events.
    pub fn sub(&self) -> u32 {
        (self.0 & 0x0000F000) >> 12
    }

    /// Returns the event minor, identifies the event in its class and subclass.
    pub fn minor(&self) -> u32 {
        self.0 & 0x00000FFF
    }
}

impl From<u32> for EventCode {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<EventCode> for u32 {
    fn from(value: EventCode) -> Self {
        value.0
    }
}

impl PartialEq<u32> for EventCode {
    fn eq(&self, other: &u32) -> bool {
        u32::from(*self).eq(other)
    }
}

impl std::ops::BitAnd<u32> for EventCode {
    type Output = EventCode;

    fn bitand(self, rhs: u32) -> Self::Output {
        Self(self.0.bitand(rhs))
    }
}

impl Debug for EventCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "EventCode {{ code: {:#010X}, main: {:#06X}, sub: {:#03X}, minor: {:#05X} }}",
            self.0,
            self.main(),
            self.sub(),
            self.minor()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::EventCode;

    #[test]
    fn full_equality_test() {
        let ecode1 = EventCode::from(0x00015003);
        let ecode2 = EventCode::from(0x00015003);

        assert_eq!(ecode1, ecode2);
        assert_eq!(ecode1.main(), ecode2.main());
        assert_eq!(ecode1.sub(), ecode2.sub());
        assert_eq!(ecode1.minor(), ecode2.minor());
    }

    #[test]
    fn equality_test() {
        let ecode1 = EventCode::from(0x00015003);
        let ecode2 = EventCode::from(0x01015003);

        assert_ne!(ecode1, ecode2);
        assert_ne!(ecode1.main(), ecode2.main());

        assert_eq!(ecode1.sub(), ecode2.sub());
        assert_eq!(ecode1.minor(), ecode2.minor());
    }
}
