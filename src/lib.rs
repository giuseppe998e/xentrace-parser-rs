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

        instance.read_file(path)?;
        Ok(instance)
    }

    pub fn get_records(&self) -> &Vec<Record> {
        &self.records
    }

    pub fn cpu_count(&self) -> u8 {
        self.cpu_domains.keys().max().map(|v| v + 1).unwrap()
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
                    Err(e) => {
                        if e.kind() != ErrorKind::Other {
                            break;
                        }
                    }
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

    fn read_u64(file: &mut File) -> Result<u64> {
        let mut buf = [0u8; 8];
        file.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf)) // host-endian because of XenTrace
    }

    fn read_event(&mut self, file: &mut File) -> Result<Event> {
        // Read header
        let hdr = Self::read_u32(file)?;
        let code = hdr & 0x0fffffff;

        // Read TSC
        let tsc = {
            let in_tsc = (hdr & (1 << 31)) > 0;
            if in_tsc {
                self.tsc_last = Self::read_u64(file)?;
            }
            self.tsc_last
        };

        // Read extras
        let extra = {
            let n_extra = (hdr >> 28) & 7;
            let mut extra = Vec::new();
            for _ in 0..n_extra {
                let val = Self::read_u32(file)?;
                extra.push(val);
            }
            extra
        };

        // Create Event
        let mut event = Event::new(code);
        event.set_extra(extra);
        event.set_tsc(tsc);
        Ok(event)
    }

    fn read_record(&mut self, file: &mut File) -> Result<Record> {
        let event = self.read_event(file)?;
        let code = event.get_code();
        let extra = event.get_extra();

        // Handle special events
        if code == TRC_TRACE_CPU_CHANGE {
            self.cpu_current = *extra.get(0).unwrap() as u8;
            return Err(Error::from(ErrorKind::Other)); // Do not save that kind of events
        } else if code == (code & TRC_SCHED_TO_RUN) {
            // XXX Move after "get current dom" ?
            let dom_u32 = *extra.get(0).unwrap();
            let dom = Domain::from_u32(dom_u32);
            self.cpu_domains.insert(self.cpu_current, dom);
        }

        // Get current domain
        let domain = self.cpu_domains.get(&self.cpu_current);
        let domain = domain.map(|d| *d).unwrap_or_default();

        // Create record
        let record = Record::new(self.cpu_current, domain, event);
        Ok(record)
    }
}
