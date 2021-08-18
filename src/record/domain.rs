#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DomainType {
    Zero,
    Idle,
    Default,
    Guest(u16),
}

impl DomainType {
    pub(crate) fn from_id(val: u16) -> Self {
        match val {
            0 => Self::Zero,
            32767 => Self::Idle,
            32768 => Self::Default,
            _ => Self::Guest(val),
        }
    }

    pub fn to_id(&self) -> u16 {
        match self {
            Self::Zero => 0,
            Self::Idle => 32767,
            Self::Default => 32768,
            Self::Guest(val) => *val,
        }
    }
}

impl Default for DomainType {
    fn default() -> Self {
        DomainType::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Domain {
    type_: DomainType, // "type" reserved
    vcpu: u16,
}

impl Domain {
    // CRATE FNs
    pub(crate) fn new(type_: DomainType, vcpu: u16) -> Self {
        Self { type_, vcpu }
    }

    pub(crate) fn from_u32(value: u32) -> Self {
        let vcpu = (value & 0x0000ffff) as u16;
        let id = (value >> 16) as u16;
        let type_ = DomainType::from_id(id);

        Self::new(type_, vcpu)
    }

    // PUBLIC FNs
    pub fn as_u32(&self) -> u32 {
        let id = (self.type_.to_id() as u32) << 16;
        let vcpu_u32 = self.vcpu as u32;

        id | vcpu_u32
    }

    pub fn get_type(&self) -> DomainType {
        self.type_
    }

    pub fn get_vcpu(&self) -> u16 {
        self.vcpu
    }
}

impl Default for Domain {
    fn default() -> Self {
        Domain::new(DomainType::default(), 0)
    }
}

/*
impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

//impl Eq for Domain {}

impl PartialOrd for Domain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Domain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dom_from_u32() {
        let dom1 = Domain::new(DomainType::Guest(1), 2);
        let dom2 = Domain::from_u32(0x00010002);

        assert_eq!(dom1, dom2);
    }

    #[test]
    fn dom_getters() {
        let dom = Domain::new(DomainType::Default, 1);

        assert_eq!(dom.as_u32(), 0x80000001);
        assert_eq!(dom.get_type(), DomainType::Default);
        assert_eq!(dom.get_vcpu(), 1);
    }

    #[test]
    fn domtype_default() {
        let domtype = DomainType::Default; // Zero or Idle

        assert_eq!(domtype, DomainType::Default);
        assert_eq!(domtype, DomainType::from_id(32768));
        assert_eq!(domtype.to_id(), 32768)
    }

    #[test]
    fn domtype_guest() {
        let domtype = DomainType::Guest(5);

        assert_eq!(domtype, DomainType::Guest(5));
        assert_eq!(domtype, DomainType::from_id(5));
        assert_eq!(domtype.to_id(), 5)
    }
}
