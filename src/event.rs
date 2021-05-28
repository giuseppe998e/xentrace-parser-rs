#[derive(Clone, Debug)]
pub(crate) struct Event {
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

    pub fn set_extra(self: &mut Self, extra: &[u32; 7]) {
        self.extra = extra.clone();
    }

    pub fn get_extra(self: Self, pos: u8) -> Option<u32> {
        self.extra[pos].copy()
    }

    pub fn has_tsc(self: Self) -> bool {
        self.in_tsc
    }

    pub fn get_tsc(self: Self) -> u64 {
        if self.in_tsc {
            return self.tsc
        }

        /*return*/0
    }
}