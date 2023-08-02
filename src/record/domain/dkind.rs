/// Type of virtual machine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl From<u16> for DomainKind {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Zero,
            32767 => Self::Idle,
            32768 => Self::Default,
            _ => Self::Guest(value),
        }
    }
}

impl From<DomainKind> for u16 {
    fn from(value: DomainKind) -> Self {
        u16::from(&value)
    }
}

impl From<&DomainKind> for u16 {
    fn from(value: &DomainKind) -> Self {
        match value {
            DomainKind::Zero => 0,
            DomainKind::Idle => 32767,
            DomainKind::Default => 32768,
            DomainKind::Guest(v) => *v,
        }
    }
}

impl PartialEq<u16> for DomainKind {
    fn eq(&self, other: &u16) -> bool {
        u16::from(*self).eq(other)
    }
}

impl PartialEq<DomainKind> for u16 {
    fn eq(&self, other: &DomainKind) -> bool {
        u16::from(*other).eq(self)
    }
}

impl Default for DomainKind {
    fn default() -> Self {
        Self::Default
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
}
