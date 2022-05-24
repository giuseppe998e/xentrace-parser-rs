#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DomainType {
    Zero,
    Idle,
    Default,
    Guest(u16),
}

impl Default for DomainType {
    fn default() -> Self {
        DomainType::Default
    }
}

impl From<u16> for DomainType {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::Zero,
            32767 => Self::Idle,
            32768 => Self::Default,
            _ => Self::Guest(val),
        }
    }
}

impl From<DomainType> for u16 {
    fn from(val: DomainType) -> Self {
        match val {
            DomainType::Zero => 0,
            DomainType::Idle => 32767,
            DomainType::Default => 32768,
            DomainType::Guest(v) => v,
        }
    }
}

impl From<DomainType> for u32 {
    fn from(val: DomainType) -> Self {
        u32::from(u16::from(val))
    }
}

impl From<DomainType> for i32 {
    fn from(val: DomainType) -> Self {
        i32::from(u16::from(val))
    }
}

impl From<DomainType> for u64 {
    fn from(val: DomainType) -> Self {
        u64::from(u16::from(val))
    }
}

impl From<DomainType> for i64 {
    fn from(val: DomainType) -> Self {
        i64::from(u16::from(val))
    }
}

impl From<DomainType> for u128 {
    fn from(val: DomainType) -> Self {
        u128::from(u16::from(val))
    }
}

impl From<DomainType> for i128 {
    fn from(val: DomainType) -> Self {
        i128::from(u16::from(val))
    }
}

impl From<DomainType> for usize {
    fn from(val: DomainType) -> Self {
        usize::from(u16::from(val))
    }
}

impl TryFrom<DomainType> for isize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: DomainType) -> Result<Self, Self::Error> {
        isize::try_from(u16::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::DomainType;

    #[test]
    fn into_u16_test() {
        let type_ = DomainType::Guest(12345);

        assert_eq!(u16::from(type_), 12345);
    }

    #[test]
    fn equality_test() {
        let type1 = DomainType::from(55u16);
        let type2 = DomainType::from(55);

        assert_eq!(type1, type2);
    }

    #[test]
    fn not_equality_test() {
        let type1 = DomainType::from(0);
        let type2 = DomainType::default();

        assert_ne!(u128::from(type1), type2.into());
    }
}
