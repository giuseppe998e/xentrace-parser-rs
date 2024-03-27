use std::{fs, io, ops::Deref, path::Path};

use self::parse::parse_trace;
use crate::{record::Record, Error, Result};

/// Represents a parsed XenTrace binary file.
///
/// The trace is truncated to the last readable record,
/// returning no errors.
///
/// # Examples
///
/// ```
/// use xentrace_parser::{Result, Trace};
///
/// fn function() -> Result<()> {
///     let trace = Trace::from_file("/path/to/xentrace.bin")?;
///
///     // Alternatively, you can create a trace from a reader:
///     // let file = std::fs::File::open("/path/to/xentrace.bin")?;
///     // let bufreader = std::io::BufReader::new(file);
///     // let trace = Trace::from_reader(bufreader);
///
///     for record in trace.iter() {
///         println!("{:?}", record);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Trace {
    records: Box<[Record]>,
    cpu_count: u32,
}

impl Trace {
    /// Constructs a `Trace` from a file specified by its path.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to open
    /// the trace file or if it fails to parse the file contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use xentrace_parser::{Result, Trace};
    ///
    /// fn main() -> Result<()> {
    ///     let trace = Trace::from_file("/path/to/xentrace.bin")?;
    ///     println!("{:?}", trace);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        fs::File::open(path)
            .map_err(|e| Error::new_source("Failed to open trace file", e))
            .map(io::BufReader::new)
            .and_then(parse_trace)
    }

    /// Constructs a `Trace` from a byte slice.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to parse the trace data.
    ///
    /// # Examples
    ///
    /// ```
    /// use xentrace_parser::{Trace, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let bytes: Vec<u8> = vec![/* byte data */];
    ///     let trace = Trace::from_bytes(&bytes)?;
    ///     println!("{:?}", trace);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self> {
        let reader = io::Cursor::new(bytes);
        parse_trace(reader)
    }

    /// Constructs a `Trace` from any type that implements `io::Read`.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to parse the trace data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use xentrace_parser::{Trace, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let file = File::open("/path/to/xentrace.bin")?;
    ///     let bufreader = BufReader::new(file);
    ///     let trace = Trace::from_reader(bufreader)?;
    ///     println!("{:?}", trace);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_reader<R: io::Read>(reader: R) -> Result<Self> {
        parse_trace(reader)
    }

    /// Returns the number of [Records](crate::record::Record) parsed from the trace file.
    ///
    /// # Examples
    ///
    /// ```
    /// use xentrace_parser::{Trace, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let trace = Trace::from_file("/path/to/xentrace.bin")?;
    ///     let record_count = trace.record_count();
    ///     println!("Record count: {}", record_count);
    ///     Ok(())
    /// }
    /// ```
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Returns the count of CPUs used in the trace file.
    ///
    /// **Note:** The value is calculated based on the
    /// number of unique CPUs found in the trace data.
    ///
    /// # Examples
    ///
    /// ```
    /// use xentrace_parser::{Trace, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let trace = Trace::from_file("/path/to/xentrace.bin")?;
    ///     let cpu_count = trace.cpu_count();
    ///     println!("CPU count: {}", cpu_count);
    ///     Ok(())
    /// }
    /// ```
    pub fn cpu_count(&self) -> u32 {
        self.cpu_count
    }
}

impl Deref for Trace {
    type Target = [Record];

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

// Functions for trace parsing logic
mod parse {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        record::{Domain, Event, EventCode, EVENT_EXTRA_CAPACITY},
        util::IoReadUtil,
    };

    const TRC_TRACE_CPU_CHANGE: u32 = 0x0001F003;
    const TRC_SCHED_TO_RUN: u32 = 0x00021F0F;

    struct ParserData {
        domains: HashMap<u32, Domain>,
        last_cpu: u32,
        records: Vec<Record>,
        last_tsc: u64,
    }

    pub(super) fn parse_trace<R: io::Read>(mut rdr: R) -> Result<Trace> {
        let mut data = ParserData {
            domains: HashMap::with_capacity(u16::BITS as usize),
            last_cpu: 0,
            records: Vec::with_capacity((u16::MAX / 2) as usize),
            last_tsc: 0,
        };

        while let Some(record) = next_record(&mut rdr, &mut data)? {
            if record.event.code == TRC_TRACE_CPU_CHANGE {
                data.last_cpu = record.event.extra[0].unwrap_or(0);
                continue;
            }

            data.records.push(record);
        }

        let records = {
            data.records.sort();
            data.records.into_boxed_slice()
        };

        match data.domains.len().try_into() {
            Ok(cpu_count) => Ok(Trace { records, cpu_count }),
            Err(_) => Err(Error::new(format_args!(
                "Failed to set host CPU count: {} > u32::MAX",
                data.domains.len()
            ))),
        }
    }

    fn next_record<R: io::Read>(rdr: &mut R, data: &mut ParserData) -> Result<Option<Record>> {
        fn read_event<R: io::Read>(rdr: &mut R, last_tsc: &mut u64) -> Result<Option<Event>> {
            // Truncate the reader at the first misread header
            let Some(header) = rdr.read_ne_u32().ok() else {
                return Ok(None);
            };

            let code = EventCode::from(header & 0x0FFFFFF);

            let tsc = {
                // has "tsc" value ?
                if header & (1 << 31) > 0 {
                    *last_tsc = rdr
                        .read_ne_u64()
                        .map_err(|e| Error::new_source("Failed to read tsc value", e))?;
                }

                *last_tsc
            };

            let extra = {
                let len = ((header >> 28) as usize) & EVENT_EXTRA_CAPACITY;
                let mut extra = [None; EVENT_EXTRA_CAPACITY];

                for entry in extra.iter_mut().take(len) {
                    *entry = rdr
                        .read_ne_u32()
                        .map(Some)
                        .map_err(|e| Error::new_source("Failed to read extra value", e))?;
                }

                extra
            };

            Ok(Some(Event { code, tsc, extra }))
        }

        // "next_record" function
        let Some(event) = read_event(rdr, &mut data.last_tsc)? else {
            return Ok(None);
        };

        let cpu = data.last_cpu;
        let domain = if event.code == (event.code & TRC_SCHED_TO_RUN) {
            let extra_0 = event.extra[0].unwrap_or(0);
            let domain = Domain::from(extra_0);
            data.domains.insert(cpu, domain);
            domain
        } else {
            data.domains.get(&cpu).copied().unwrap_or_default()
        };

        Ok(Some(Record { cpu, domain, event }))
    }
}
