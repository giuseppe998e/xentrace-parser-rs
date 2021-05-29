mod record;
pub use record::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct XTParser {
    // Host CPUs fiels
    cpu_current: u8,
    cpu_highter: u8,
    // Virt. domain fields (CPU, Domain)
    dom_hmap: HashMap<u8, Domain>,
    // Events fiels
    tsc_last: u64,
    events: Vec<Record>,
}

impl XTParser {
    pub fn new(path: &str) -> Self {
        let mut instance = Self {
            // Host CPUs fiels
            cpu_current: 0,
            cpu_highter: 0,
            // Virt. domain fields (CPU, Domain)
            dom_hmap: HashMap::new(),
            // Events fiels
            tsc_last: 0,
            events: Vec::new(),
        };

        let _ = instance.from_file(path).unwrap();
        instance
    }

    fn from_file(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;

        loop {
            let record = self.new_record(&mut file);
            match record {
                Ok(val) => self.events.push(val),
                Err(true) => return Err(Error::from(ErrorKind::UnexpectedEof)),
                Err(false) => return Ok(()), // EOF
            }
        }
    }

    fn upd_host_cpu(&mut self, cpu: u8) {
        self.cpu_current = cpu;

        if self.cpu_highter < cpu {
            self.cpu_highter = cpu;
        }
    }

    fn upd_last_tsc(&mut self, tsc: u64) {
        self.tsc_last = tsc;
    }

    fn new_record(&mut self, file: &mut File) -> Result<Record, bool> {
        // Read event header

        Err(false)
    }
}
