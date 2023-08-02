use std::{
    fmt::{Debug, Formatter, Result},
    ops::BitAnd,
};

/// Contains the event code read as a 32-bit unsigned big-endian integer.
#[repr(transparent)]
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

impl From<&EventCode> for u32 {
    fn from(value: &EventCode) -> Self {
        value.0
    }
}

impl PartialEq<u32> for EventCode {
    fn eq(&self, other: &u32) -> bool {
        u32::from(*self).eq(other)
    }
}

impl PartialEq<EventCode> for u32 {
    fn eq(&self, other: &EventCode) -> bool {
        u32::from(*other).eq(self)
    }
}

impl BitAnd<u32> for EventCode {
    type Output = EventCode;

    fn bitand(self, rhs: u32) -> Self::Output {
        Self(self.0.bitand(rhs))
    }
}

impl BitAnd<EventCode> for u32 {
    type Output = EventCode;

    fn bitand(self, rhs: EventCode) -> Self::Output {
        EventCode(rhs.0.bitand(self))
    }
}

impl Debug for EventCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("EventCode")
            .field("value", &format_args!("{:#010X}", self.0))
            .field("main", &format_args!("{:#06X}", self.main()))
            .field("sub", &format_args!("{:#03X}", self.sub()))
            .field("minor", &format_args!("{:#05X}", self.minor()))
            .finish()
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
