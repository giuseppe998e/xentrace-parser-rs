use crate::record::Record;

#[derive(Debug, Default)]
pub struct Trace {
    pub records: Box<[Record]>,
    pub cpus: Box<[u16]>,
}

impl Trace {
    pub fn cpu_count(&self) -> u16 {
        self.cpus.iter().max().map(|v| v + 1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Trace;

    #[test]
    fn cpu_count_test() {
        let trace = Trace {
            records: Box::new([]),
            cpus: Box::new([1, 5, 6, 7, 2, 3, 8, 4]),
        };

        assert_eq!(trace.cpu_count(), 9);
    }
}
