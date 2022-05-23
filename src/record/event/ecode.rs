use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Copy, Eq)]
pub struct EventCode {
    code: u32,
    pub main: u16,
    pub sub: u8,
    pub minor: u16,
}

impl EventCode {
    pub fn from_u32(val: u32) -> Self {
        Self {
            code: val,
            main: ((val & 0x0FFF0000) >> 16) as u16,
            sub: ((val & 0x0000F000) >> 12) as u8,
            minor: (val & 0x00000FFF) as u16,
        }
    }

    pub fn into_u32(&self) -> u32 {
        self.code
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
        EventCode::from_u32(val)
    }
}

impl Into<u32> for EventCode {
    fn into(self) -> u32 {
        self.into_u32()
    }
}

#[cfg(test)]
mod tests {
    use super::EventCode;

    #[test]
    fn equality_test() {
        let ecode1 = EventCode::from(0x00015003);
        let ecode2 = EventCode::from_u32(0x00015003);

        assert_eq!(ecode1, ecode2);
        assert_eq!(ecode1.main, ecode2.main);
        assert_eq!(ecode1.sub, ecode2.sub);
        assert_eq!(ecode1.minor, ecode2.minor);
    }

    #[test]
    fn not_full_equality_test() {
        let ecode1 = EventCode::from_u32(0x00015003);
        let ecode2 = EventCode::from(0x01015003);

        assert_ne!(ecode1, ecode2);
        assert_ne!(ecode1.main, ecode2.main);
        
        assert_eq!(ecode1.sub, ecode2.sub);
        assert_eq!(ecode1.minor, ecode2.minor);
    }
}
