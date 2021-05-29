#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
            Self::Guest(v) => *v,
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Domain {
    tipe: DomainType, // "type" reserved
    vcpu: u16,
}

impl Domain {
    // CRATE FNs
    pub(crate) fn new(id: u16, vcpu: u16) -> Self {
        Self {
            tipe: DomainType::from_id(id),
            vcpu,
        }
    }

    pub(crate) fn from_u32(value: u32) -> Self {
        let id = (value & 0x0000ffff) as u16;
        let vcpu = (value >> 16) as u16;
        Self::new(id, vcpu)
    }

    // PUBLIC FNs
    pub fn as_u32(&self) -> u32 {
        let id = self.tipe.to_id();
        ((self.vcpu << 16) | id).into()
    }

    pub fn get_type(&self) -> DomainType {
        self.tipe
    }

    pub fn get_vcpu(&self) -> u16 {
        self.vcpu
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
