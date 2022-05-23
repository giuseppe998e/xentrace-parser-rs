mod dtype;
pub use dtype::DomainType;

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Domain {
    pub type_: DomainType,
    pub vcpu: u16,
}

impl Domain {
    pub fn from_u32(val: u32) -> Self {
        let vcpu = (val & 0x0000FFFF) as u16;
        let type_ = {
            let id = (val >> 16) as u16;
            DomainType::from_u16(id)
        };

        Self { type_, vcpu }
    }

    pub fn into_u32(&self) -> u32 {
        let type_u32 = self.type_.into_u16() as u32;
        let vcpu_u32 = self.vcpu as u32;

        (type_u32 << 16) | vcpu_u32
    }
}

impl From<u32> for Domain {
    fn from(val: u32) -> Self {
        Domain::from_u32(val)
    }
}

impl Into<u32> for Domain {
    fn into(self) -> u32 {
        self.into_u32()
    }
}

impl Into<u64> for Domain {
    fn into(self) -> u64 {
        u64::from(self.into_u32())
    }
}

impl Into<i64> for Domain {
    fn into(self) -> i64 {
        i64::from(self.into_u32())
    }
}

impl Into<u128> for Domain {
    fn into(self) -> u128 {
        u128::from(self.into_u32())
    }
}

impl Into<i128> for Domain {
    fn into(self) -> i128 {
        i128::from(self.into_u32())
    }
}

impl TryInto<usize> for Domain {
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> std::result::Result<usize, Self::Error> {
        usize::try_from(self.into_u32())
    }
}

impl TryInto<isize> for Domain {
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> std::result::Result<isize, Self::Error> {
        isize::try_from(self.into_u32())
    }
}

#[cfg(test)]
mod tests {
    use super::Domain;

    #[test]
    fn equality_test() {
        let dom1 = Domain::from(0x00015003);
        let dom2 = Domain::from_u32(0x00015003);

        assert_eq!(dom1, dom2);
        assert_eq!(dom1.type_, dom2.type_);
        assert_eq!(dom1.vcpu, dom2.vcpu);
    }

    #[test]
    fn not_equality_test() {
        let dom1 = Domain::from_u32(0x00015003);
        let dom2 = Domain::from(0x00015103);

        assert_ne!(dom1.into_u32(), dom2.into());
        assert_ne!(dom1.vcpu, dom2.vcpu);

        assert_eq!(dom1.type_, dom2.type_);
    }
}
