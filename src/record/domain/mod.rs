mod dtype;
pub use dtype::DomainKind;

/// Contains the domain information of the [`Record`](super::Record).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Domain {
    /// The virtual processor number.
    pub(crate) vcpu: u16,
    /// The [type](dtype::DomainType) of virtual machine.
    pub(crate) kind: DomainKind,
}

impl Domain {
    /// Returns the virtual processor number.
    pub fn virtual_cpu(&self) -> u16 {
        self.vcpu
    }

    /// Returns the [type](dtype::DomainType) of virtual machine.
    pub fn kind(&self) -> DomainKind {
        self.kind
    }
}

impl From<u32> for Domain {
    fn from(value: u32) -> Self {
        let vcpu = (value & 0x0000FFFF) as u16;
        let kind = {
            let id = (value >> 16) as u16;
            DomainKind::from(id)
        };

        Self { vcpu, kind }
    }
}

impl From<Domain> for u32 {
    fn from(value: Domain) -> Self {
        u32::from(&value)
    }
}

impl From<&Domain> for u32 {
    fn from(value: &Domain) -> Self {
        let vcpu = u32::from(value.vcpu);
        let kind = u32::from(u16::from(value.kind));

        (kind << 16) | vcpu
    }
}

impl PartialEq<u32> for Domain {
    fn eq(&self, other: &u32) -> bool {
        u32::from(*self).eq(other)
    }
}

impl PartialEq<Domain> for u32 {
    fn eq(&self, other: &Domain) -> bool {
        u32::from(*other).eq(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Domain;

    #[test]
    fn full_equality_test() {
        let dom1 = Domain::from(0x00015003);
        let dom2 = Domain::from(0x00015003);

        assert_eq!(dom1, dom2);
        assert_eq!(dom1.kind, dom2.kind);
        assert_eq!(dom1.vcpu, dom2.vcpu);
    }

    #[test]
    fn equality_test() {
        let dom1 = Domain::from(0x00015003);
        let dom2 = Domain::from(0x00015103);

        assert_ne!(dom1, dom2);
        assert_ne!(dom1.vcpu, dom2.vcpu);

        assert_eq!(dom1.kind, dom2.kind);
    }
}
