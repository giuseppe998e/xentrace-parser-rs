/// Type of virtual machine.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DomainKind {
    /// The zero/host domain (*The privileged VM*).
    Zero,
    /// The idle domain (*The CPU is not used*).
    Idle,
    /// The default domain (*No info available*).
    Default,
    /// The unprivileged domain (*The running VMs*).
    Guest(u16),
}

impl Default for DomainKind {
    fn default() -> Self {
        Self::Default
    }
}

impl From<u16> for DomainKind {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::Zero,
            32767 => Self::Idle,
            32768 => Self::Default,
            _ => Self::Guest(val),
        }
    }
}

impl From<DomainKind> for u16 {
    fn from(val: DomainKind) -> Self {
        match val {
            DomainKind::Zero => 0,
            DomainKind::Idle => 32767,
            DomainKind::Default => 32768,
            DomainKind::Guest(v) => v,
        }
    }
}

impl From<DomainKind> for u32 {
    fn from(val: DomainKind) -> Self {
        u32::from(u16::from(val))
    }
}

impl From<DomainKind> for i32 {
    fn from(val: DomainKind) -> Self {
        i32::from(u16::from(val))
    }
}

impl From<DomainKind> for u64 {
    fn from(val: DomainKind) -> Self {
        u64::from(u16::from(val))
    }
}

impl From<DomainKind> for i64 {
    fn from(val: DomainKind) -> Self {
        i64::from(u16::from(val))
    }
}

impl From<DomainKind> for u128 {
    fn from(val: DomainKind) -> Self {
        u128::from(u16::from(val))
    }
}

impl From<DomainKind> for i128 {
    fn from(val: DomainKind) -> Self {
        i128::from(u16::from(val))
    }
}

impl From<DomainKind> for usize {
    fn from(val: DomainKind) -> Self {
        usize::from(u16::from(val))
    }
}

impl TryFrom<DomainKind> for isize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: DomainKind) -> Result<Self, Self::Error> {
        isize::try_from(u16::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::DomainKind;

    #[test]
    fn into_u16_test() {
        let type_ = DomainKind::Guest(12345);

        assert_eq!(u16::from(type_), 12345);
    }

    #[test]
    fn equality_test() {
        let type1 = DomainKind::from(55u16);
        let type2 = DomainKind::from(55);

        assert_eq!(type1, type2);
    }

    #[test]
    fn not_equality_test() {
        let type1 = DomainKind::from(0);
        let type2 = DomainKind::default();

        assert_ne!(u128::from(type1), type2.into());
    }
}
