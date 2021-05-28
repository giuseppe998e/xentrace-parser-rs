#[derive(Clone, Debug)]
pub struct Domain(u32);

impl Domain {
    pub fn new(id: u16, vcpu: u16) -> Self {
        let shift_vcpu = (vcpu as u32) << 16;
        let value = shift_vcpu | (id as u32);
        Self(value)
    }

    pub fn from_u32(value: u32) -> Self {
        Self(value)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn get_id(&self) -> u16 {
        (self.0 & 0x0000ffff) as u16
    }

    pub fn get_vcpu(&self) -> u16 {
        (self.0 >> 16) as u16
    }
}
