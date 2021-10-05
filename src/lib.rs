pub mod record;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::Path;

use record::*;

pub const TRC_TRACE_CPU_CHANGE: u32 = 0x0001f003;
pub const TRC_SCHED_TO_RUN: u32 = 0x00021f0f;

#[derive(Debug)]
pub struct Parser {
    // Host CPUs fiels
    cpu_current: u16,
    cpu_domains: HashMap<u16, Domain>,
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

        {
            let path_i = Path::new(path);
            let mut file = File::open(path_i)?;

            loop {
                let record = instance.read_record(&mut file);
                match record {
                    Ok(r) => instance.records.push(r),
                    Err(e) if e.kind() != ErrorKind::Other => break,
                    Err(_) => (),
                }
            }
        } // File closed

        instance.records.sort_unstable();
        Ok(instance)
    }

    pub fn get_records(&self) -> &Vec<Record> {
        &self.records
    }

    pub fn cpu_count(&self) -> u16 {
        self.cpu_domains.keys().max().map(|v| v + 1).unwrap()
    }

    // PRIVATE FNs
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
        let code = hdr & 0x0FFFFFFF;

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
        Ok(Event::new(code, tsc, extra))
    }

    fn read_record(&mut self, file: &mut File) -> Result<Record> {
        let event = self.read_event(file)?;
        let code = event.get_code().into_u32();
        let extra = event.get_extra();

        // Handle TRC_TRACE_CPU_CHANGE event
        if code == TRC_TRACE_CPU_CHANGE {
            self.cpu_current = *extra.get(0).unwrap() as u16;
            return Err(Error::from(ErrorKind::Other)); // Do not save that kind of events
        }

        let domain = *self.cpu_domains.entry(self.cpu_current).or_insert_with(|| {
            let dom = *extra.get(0).unwrap();
            Domain::from_u32(dom)
        });

        // Create record
        Ok(Record::new(self.cpu_current, domain, event))
    }
}
