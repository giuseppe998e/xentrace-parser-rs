use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Result},
    path::Path,
};

pub mod codes;
use codes::*;

pub mod record;
use record::{Domain, Event, EventCode, Record, EVENT_EXTRA_MAXLEN};

mod trace;
pub use trace::Trace;

mod util;
use util::{read_u32, read_u64};

/// Returns the [Trace](trace::Trace) structure, enclosed in an [std::io:Result](std::io::Result),
/// containing the XenTrace binary file records parsed.
///
/// Records are sorted according to their `tsc` value.
///
/// # Examples
///
/// ```
/// use std::io::Result;
/// use xentrace_parser::{xentrace_parse, Trace};
///
/// fn function() -> Result<()> {
///     let trace: Trace = xentrace_parse("/path/to/trace.xen.dat")?;
///
///     Ok(())
/// }
/// ```
pub fn xentrace_parse(file: &str) -> Result<Trace> {
    let mut records = Vec::<Record>::new();
    let cpu_count: u16 = {
        let path = Path::new(file);
        let mut file = File::open(path)?;

        let mut last_tsc = 0u64;
        let mut current_cpu = 0u16;
        let mut cpus_dom = HashMap::<u16, Domain>::new();

        loop {
            let record = parse_record(&mut file, &mut last_tsc, &mut current_cpu, &mut cpus_dom);
            match record {
                Ok(r) => records.push(r),
                Err(e) => match e.kind() {
                    ErrorKind::Other => (),
                    _ => break,
                },
            };
        }

        cpus_dom.keys().max().map(|&v| v + 1).unwrap_or(0)
    }; // "file" closes here

    records.sort();

    Ok(Trace(records.into_boxed_slice(), cpu_count))
}

fn parse_event(file: &mut File, last_tsc: &mut u64) -> Result<Event> {
    let hdr = read_u32(file)?;

    // Code
    let code = hdr & 0x0FFFFFFF;
    let code = EventCode::from(code);

    // T.S.C.
    let tsc = {
        let in_tsc = (hdr & (1 << 31)) > 0;
        if in_tsc {
            *last_tsc = read_u64(file)?;
        }

        *last_tsc
    };

    // Extra list
    let extra = {
        let n_extra = ((hdr >> 28) as usize) & EVENT_EXTRA_MAXLEN;
        let mut extra = [None; EVENT_EXTRA_MAXLEN];

        if n_extra > 0 {
            for e in extra.iter_mut().take(n_extra) {
                *e = Some(read_u32(file)?);
            }
        }

        extra
    };

    Ok(Event { code, tsc, extra })
}

fn parse_record(
    file: &mut File,
    last_tsc: &mut u64,
    current_cpu: &mut u16,
    cpus_dom: &mut HashMap<u16, Domain>,
) -> Result<Record> {
    let event = parse_event(file, last_tsc)?;
    let code = u32::from(event.code);

    if code == TRC_TRACE_CPU_CHANGE {
        *current_cpu = event.extra[0].unwrap_or(0) as u16;
        return Err(Error::from(ErrorKind::Other)); // Do not save this kind of events
    }

    let domain = match code == (code & TRC_SCHED_TO_RUN) {
        true => {
            let extra_0 = event.extra[0].unwrap_or(0);
            let dom = Domain::from(extra_0);
            cpus_dom.insert(*current_cpu, dom);
            Some(dom)
        }
        false => cpus_dom.get(current_cpu).copied(),
    }
    .unwrap_or_default();

    Ok(Record {
        cpu: *current_cpu,
        domain,
        event,
    })
}
