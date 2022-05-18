use crate::record::Record;

#[derive(Debug, Default)]
pub struct Trace {
    pub records: Vec<Record>,
    pub cpus: Vec<u16>,
}

impl Trace {
    pub fn cpu_count(&self) -> u16 {
        self.cpus.iter().max().map(|v| v + 1).unwrap()
    }
}
