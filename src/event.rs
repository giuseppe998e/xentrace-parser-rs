use std::cmp::Ordering;

#[derive(Clone, Eq, Debug)]
pub struct Event {
    code: u32,
    tsc: Option<u64>,
    extra: [Option<u32>; 7],
}

impl Event {
    // CRATE FNs
    pub(crate) fn new(code: u32) -> Self {
        Self {
            code,
            tsc: None,
            extra: [None; 7],
        }
    }

    pub(crate) fn set_extra(&self, extra: [Option<u32>; 7]) {
        self.extra = extra;
    }

    pub(crate) fn set_tsc(&self, value: u64) {
        self.tsc = Some(value);
    }

    // PUBLIC FNs
    pub fn get_code(&self) -> u32 {
        self.code
    }

    pub fn get_extra_size(&self) -> u8 {
        self.extra.iter().filter(|x| x.is_some()).count() as u8
    }

    pub fn get_extra(&self) -> [Option<u32>; 7] {
        self.extra.clone()
    }

    pub fn get_tsc(&self) -> Option<u64> {
        self.tsc
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
