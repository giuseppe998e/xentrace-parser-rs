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
            DomainType::from_id(id)
        };

        Self { type_, vcpu }
    }

    pub fn into_u32(&self) -> u32 {
        let type_id = self.type_.into_id() as u32;
        let vcpu_u32 = self.vcpu as u32;

        (type_id << 16) | vcpu_u32
    }
}
