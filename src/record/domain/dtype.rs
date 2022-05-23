#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DomainType {
    Zero,
    Idle,
    Default,
    Guest(u16),
}

impl DomainType {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0 => Self::Zero,
            32767 => Self::Idle,
            32768 => Self::Default,
            _ => Self::Guest(val),
        }
    }

    pub fn into_u16(&self) -> u16 {
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

impl From<u16> for DomainType {
    fn from(v: u16) -> Self {
        DomainType::from_u16(v)
    }
}

impl Into<u16> for DomainType {
    fn into(self) -> u16 {
        self.into_u16()
    }
}

#[cfg(test)]
mod tests {
    use super::DomainType;

    #[test]
    fn into_u16_test() {
        let type_ = DomainType::Guest(12345);
        
        assert_eq!(type_.into_u16(), 12345);
    }

    #[test]
    fn equality_test() {
        let type1 = DomainType::from_u16(55);
        let type2 = DomainType::from(55);

        assert_eq!(type1, type2);
    }

    #[test]
    fn not_equality_test() {
        let type1 = DomainType::from(0);
        let type2 = DomainType::Default;

        assert_ne!(type1.into_u16(), type2.into());
    }
}
