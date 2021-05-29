mod record;
pub use record::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

const TRC_TRACE_CPU_CHANGE: u32 = 0x0001f003;
const TRC_SCHED_MIN: u32 = 0x00021000;

#[derive(Debug)]
pub struct XTParser {
    // Host CPUs fiels
    cpu_current: u8,
    cpu_highter: u8,
    cpu_domains: HashMap<u8, Domain>,
    // Events fiels
    tsc_last: u64,
    events: Vec<Record>,
}

impl XTParser {
    // PUBLIC FNs
    pub fn new(path: &str) -> Result<Self> {
        let mut instance = Self {
            // Host CPUs fiels
            cpu_current: 0,
            cpu_highter: 0,
            cpu_domains: HashMap::new(),
            // Events fiels
            tsc_last: 0,
            events: Vec::new(),
        };

        let _ = instance.read_file(path)?;
        Ok(instance)
    }

    pub fn get_events(&self) -> &Vec<Record> {
        &self.events
    }

    pub fn get_cpu_count(&self) -> u8 {
        self.cpu_current + 1
    }

    // PRIVATE FNs
    fn read_file(&mut self, path: &str) -> Result<()> {
        let mut file = File::open(path)?;

        loop {
            match self.read_record(&mut file) {
                Ok(val) => self.events.push(val),
                Err(_) => break,
            }
        }

        self.events.sort();
        Ok(())
    }

    fn update_cpu(&mut self, cpu: u8) {
        self.cpu_current = cpu;

        if self.cpu_highter < cpu {
            self.cpu_highter = cpu;
        }
    }

    fn update_cpu_dom(&mut self, cpu: u8, dom_u32: u32) -> Option<Domain> {
        let dom = Domain::from_u32(dom_u32);
        self.cpu_domains.insert(cpu, dom)
    }

    fn read_u32(file: &mut File) -> Result<u32> {
        let mut buf = [0u8; 4];
        file.read(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_tsc(hdr: u32, file: &mut File) -> Result<Option<u64>> {
        let in_tsc = (hdr & (1 << 31)) != 0;
        match in_tsc {
            false => Ok(None),
            true => {
                let mut buf = [0u8; 8];
                file.read(&mut buf)?;
                Ok(Some(u64::from_be_bytes(buf)))
            }
        }
    }

    fn read_extra(hdr: u32, file: &mut File) -> Result<Vec<u32>> {
        let n_extra = (hdr >> 28) & 0x7;
        let mut extra: Vec<u32> = vec![];

        for _ in 0..n_extra {
            let value = Self::read_u32(file)?;
            extra.push(value);
        }

        Ok(extra)
    }

    fn read_record(&mut self, file: &mut File) -> Result<Record> {
        let hdr = Self::read_u32(file)?;

        let code = hdr & 0x0fffffff;
        let tsc = Self::read_tsc(hdr, file)?;
        let extra = Self::read_extra(hdr, file)?;

        // Check TRC_TRACE_CPU_CHANGE event
        if code == TRC_TRACE_CPU_CHANGE {
            let cpu = (*extra.get(0).unwrap()) as u8;
            self.update_cpu(cpu);
        } else {
            // Check TRC_SCHED_MIN family
            let is_sched_min = code == (code & (TRC_SCHED_MIN | 0xf0f));
            if is_sched_min {
                self.update_cpu_dom(self.cpu_current, *extra.get(0).unwrap());
            }
        }

        let mut event = Event::new(code);
        event.set_extra(extra.as_slice());
        if let Some(val) = tsc {
            self.tsc_last = val;
            event.set_tsc(val);
        } else {
            event.set_tsc(self.tsc_last);
        }

        let dom_current = self.cpu_domains.get(&self.cpu_current);
        let dom_current = match dom_current {
            Some(&v) => v,
            None => Domain::default(),
        };

        let record = Record::new(self.cpu_current, dom_current, event);
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_print() {
        let parser = XTParser::new("/home/giuseppe/Downloads/trace_xen.bin").unwrap();
        let records = parser.get_events();

        for rec in records {
            println!("{:?}", rec);
        }
    }
}
