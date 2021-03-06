mod dtype;
pub use dtype::DomainKind;

/// Contains the domain information of the [`Record`](super::Record).
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Domain {
    /// The [type](dtype::DomainType) of virtual machine.
    pub(crate) kind: DomainKind,
    /// The virtual processor number.
    pub(crate) vcpu: u16,
}

impl Domain {
    /// Returns the [type](dtype::DomainType) of virtual machine.
    pub fn kind(&self) -> DomainKind {
        self.kind
    }

    /// Returns the virtual processor number.
    pub fn virt_cpu(&self) -> u16 {
        self.vcpu
    }
}

impl From<u32> for Domain {
    fn from(val: u32) -> Self {
        let vcpu = (val & 0x0000FFFF) as u16;
        let kind = {
            let id = (val >> 16) as u16;
            DomainKind::from(id)
        };

        Self { kind, vcpu }
    }
}

impl From<Domain> for u32 {
    fn from(val: Domain) -> Self {
        let kind_u32 = u32::from(val.kind);
        let vcpu_u32 = val.vcpu as u32;

        (kind_u32 << 16) | vcpu_u32
    }
}

impl From<Domain> for u64 {
    fn from(val: Domain) -> Self {
        u64::from(u32::from(val))
    }
}

impl From<Domain> for i64 {
    fn from(val: Domain) -> Self {
        i64::from(u32::from(val))
    }
}

impl From<Domain> for u128 {
    fn from(val: Domain) -> Self {
        u128::from(u32::from(val))
    }
}

impl From<Domain> for i128 {
    fn from(val: Domain) -> Self {
        i128::from(u32::from(val))
    }
}

impl TryFrom<Domain> for usize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: Domain) -> std::result::Result<Self, Self::Error> {
        usize::try_from(u32::from(value))
    }
}

impl TryFrom<Domain> for isize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: Domain) -> std::result::Result<Self, Self::Error> {
        isize::try_from(u32::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::Domain;

    #[test]
    fn equality_test() {
        let dom1 = Domain::from(0x00015003);
        let dom2 = Domain::from(0x00015003);

        assert_eq!(dom1, dom2);
        assert_eq!(dom1.kind, dom2.kind);
        assert_eq!(dom1.vcpu, dom2.vcpu);
    }

    #[test]
    fn not_equality_test() {
        let dom1 = Domain::from(0x00015003);
        let dom2 = Domain::from(0x00015103);

        assert_ne!(u32::from(dom1), dom2.into());
        assert_ne!(dom1.vcpu, dom2.vcpu);

        assert_eq!(dom1.kind, dom2.kind);
    }
}
