mod record;
pub use record::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::Path;

pub const TRC_TRACE_CPU_CHANGE: u32 = 0x0001f003;
pub const TRC_SCHED_TO_RUN: u32 = 0x00021f0f;

#[derive(Debug)]
pub struct Parser {
    // Host CPUs fiels
    cpu_current: u8,
    cpu_domains: HashMap<u8, Domain>,
    // Records fiels
    tsc_last: u64,
    records: Vec<Record>,
}

impl Parser {
    // PUBLIC FNs
    pub fn new(path: &str) -> Result<Self> {
        let mut instance = Self {
            // Host CPUs fiels
            cpu_current: 0,
            cpu_domains: HashMap::new(),
            // Records fiels
            tsc_last: 0,
            records: Vec::new(),
        };

        let _ = instance.read_file(path)?;
        Ok(instance)
    }

    pub fn get_records(&self) -> &Vec<Record> {
        &self.records
    }

    pub fn cpu_count(&self) -> Option<u8> {
        self.cpu_domains.keys().max().map(|v| v + 1)
    }

    // PRIVATE FNs
    fn read_file(&mut self, path: &str) -> Result<()> {
        {
            let path_i = Path::new(path);
            let mut file = File::open(path_i)?;

            loop {
                let record = self.read_record(&mut file);
                match record {
                    Ok(r) => self.records.push(r),
                    Err(e) => match e.kind() {
                        ErrorKind::Other => {}
                        _ => break,
                    },
                }
            }
        } // File closed

        self.records.sort_unstable();
        Ok(())
    }

    fn read_u32(file: &mut File) -> Result<u32> {
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf)?;
        Ok(u32::from_ne_bytes(buf)) // host-endian because of XenTrace
    }

    fn read_tsc(hdr: u32, file: &mut File) -> Result<Option<u64>> {
        let in_tsc = (hdr & (1 << 31)) > 0;
        let tsc = match in_tsc {
            true => {
                let mut buf = [0u8; 8];
                file.read_exact(&mut buf)?;
                Some(u64::from_ne_bytes(buf)) // host-endian because of XenTrace
            }
            false => None,
        };

        Ok(tsc)
    }

    fn read_extra(hdr: u32, file: &mut File) -> Result<Vec<u32>> {
        let n_extra = (hdr >> 28) & 7;
        let mut extra: Vec<u32> = Vec::new();

        for _ in 0..n_extra {
            let value = Self::read_u32(file)?;
            extra.push(value);
        }

        Ok(extra)
    }

    fn read_record(&mut self, file: &mut File) -> Result<Record> {
        // Read header
        let hdr = Self::read_u32(file)?;

        // Read event data
        let code = hdr & 0x0fffffff;
        let tsc = Self::read_tsc(hdr, file)?;
        let extra = Self::read_extra(hdr, file)?;

        // Handle special events
        if code == TRC_TRACE_CPU_CHANGE {
            let cpu = *extra.get(0).unwrap() as u8;
            self.cpu_current = cpu;
            return Err(Error::from(ErrorKind::Other)); // Do not save
        } else {
            let is_sched_min = code == (code & TRC_SCHED_TO_RUN);
            if is_sched_min {
                let dom_u32 = *extra.get(0).unwrap();
                let dom = Domain::from_u32(dom_u32);
                self.cpu_domains.insert(self.cpu_current, dom);
            }
        }

        // Create event
        let mut event = Event::new(code);
        event.set_extra(extra);
        match tsc {
            None => event.set_tsc(self.tsc_last),
            Some(v) => {
                self.tsc_last = v;
                event.set_tsc(v);
            }
        }

        // Get current domain
        let domain = self.cpu_domains.get(&self.cpu_current);
        let domain = match domain {
            Some(&v) => v,
            None => Domain::default(),
        };

        // Create record
        let record = Record::new(self.cpu_current, domain, event);
        Ok(record)
    }
}
