use std::cmp::Ordering;

#[derive(Clone, Eq, Debug)]
pub struct Event {
    code: u32,
    in_tsc: bool,
    tsc: u64,
    n_extra: u8, // 0..7
    extra: [u32; 7],
}

impl Event {
    // CRATE FNs
    pub(crate) fn new(code: u32) -> Self {
        Self {
            code,
            n_extra: extra.len() as u8,
            in_tsc: tsc > 0,
            tsc,
            extra,
        }
    }

    pub(crate) fn set_extra(&self, extra: [u32; 7]) {
        self.n_extra = extra.len() as u8;
        self.extra = extra;
    }

    pub(crate) fn set_tsc(&self, value: u64) {
        self.in_tsc = true;
        self.tsc = value;
    }

    // PUBLIC FNs
    pub fn get_code(&self) -> u32 {
        self.code
    }

    pub fn get_extra_size(&self) -> u8 {
        self.n_extra
    }

    pub fn get_extra(&self) -> [u32; 7] {
        self.extra.clone()
    }

    pub fn get_tsc(&self) -> Option<u64> {
        /*return*/
        match self.in_tsc {
            true => Some(self.tsc),
            false => None,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.tsc == other.tsc
            && self
                .extra
                .iter()
                .zip(other.extra.iter())
                .all(|(a, b)| a == b)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tsc.cmp(&other.tsc)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
