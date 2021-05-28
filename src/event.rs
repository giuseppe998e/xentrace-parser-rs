#[derive(Clone, Debug)]
pub struct Event {
    code: u32,
    in_tsc: bool,
    tsc: u64,
    extra: [u32; 7],
}

impl Event {
    pub fn new(code: u32, in_tsc: bool, tsc: u64) -> Self {
        Self {
            code,
            in_tsc,
            tsc,
            extra: [0; 7],
        }
    }

    pub fn set_extra(&mut self, extra: &[u32; 7]) {
        self.extra = extra.clone();
    }

    pub fn get_extra(&self, pos: u8) -> Option<u32> {
        self.extra[pos].copy()
    }

    pub fn get_tsc(&self) -> Option<u64> {
        /*return*/
        match self.in_tsc {
            true => Some(self.tsc),
            false => None,
        }
    }
}
