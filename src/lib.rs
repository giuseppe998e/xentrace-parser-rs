pub mod codes;
use codes::*;

pub mod record;
use record::{Domain, Event, EventCode, Record, EVENT_EXTRA_MAXLEN};

mod trace;
pub use trace::Trace;

use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Read, Result},
    path::Path,
};

pub fn xentrace_parse(path: &str) -> Result<Trace> {
    let mut records = Vec::<Record>::new();
    let cpus: Vec<u16>;

    {
        let path_i = Path::new(path);
        let mut file = File::open(path_i)?;

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
            }
        }

        cpus = cpus_dom.keys().copied().collect()
    } // "file" closes here

    records.sort();

    Ok(Trace {
        records: records.into_boxed_slice(),
        cpus: cpus.into_boxed_slice(),
    })
}

#[inline]
fn read_u32(file: &mut File) -> Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_ne_bytes(buf)) // host-endian because of XenTrace
}

#[inline]
fn read_u64(file: &mut File) -> Result<u64> {
    let mut buf = [0u8; 8];
    file.read_exact(&mut buf)?;
    Ok(u64::from_ne_bytes(buf)) // host-endian because of XenTrace
}

fn parse_event(file: &mut File, last_tsc: &mut u64) -> Result<Event> {
    let hdr = read_u32(file)?;

    // Code
    let code = hdr & 0x0FFFFFFF;
    let code = EventCode::from_u32(code);

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
                *e = Some(read_u32(file)?)
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
    let code = event.code.into_u32();

    if code == TRC_TRACE_CPU_CHANGE {
        *current_cpu = event.extra[0].unwrap_or(0) as u16;
        return Err(Error::from(ErrorKind::Other)); // Do not save this kind of events
    }

    let domain = match code == (code & TRC_SCHED_TO_RUN) {
        true => {
            let extra_0 = event.extra[0].unwrap_or(0);
            let dom = Domain::from_u32(extra_0);
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
