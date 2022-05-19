pub mod codes;
use codes::*;

pub mod record;
use record::{Domain, Event, EventCode, Record};

mod trace;
pub use trace::Trace;

use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Read, Result},
    path::Path,
};

pub fn xentrace_parse(path: &str) -> Result<Trace> {
    let mut trace = Trace::default();

    {
        let path_i = Path::new(path);
        let mut file = File::open(path_i)?;

        let mut last_tsc = 0u64;
        let mut current_cpu = 0u16;
        let mut cpus_dom = HashMap::<u16, Domain>::new();

        loop {
            let record = parse_record(&mut file, &mut last_tsc, &mut current_cpu, &mut cpus_dom);
            match record {
                Ok(r) => trace.records.push(r),
                Err(e) => match e.kind() {
                    ErrorKind::Other => (),
                    _ => break,
                }
            }
        }

        trace.cpus = cpus_dom.keys().map(|v| *v).collect()
    } // "file" closes here

    trace.records.sort_unstable();
    Ok(trace)
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

fn parse_event(file: &mut File, last_tsc: &mut u64) -> Result<Event> {
    let hdr = read_u32(file)?;

    // Event code
    let code = hdr & 0x0FFFFFFF;

    // Event tsc
    let tsc = {
        let in_tsc = (hdr & (1 << 31)) > 0;
        if in_tsc {
            *last_tsc = read_u64(file)?;
        }
        *last_tsc
    };

    // Event extras
    let extra = {
        let n_extra = (hdr >> 28) & 7;
        let mut extra = Vec::with_capacity(n_extra as usize);
        for _ in 0..n_extra {
            let val = read_u32(file)?;
            extra.push(val);
        }

        extra
    };

    Ok(Event {
        code: EventCode::from_u32(code),
        tsc,
        extra,
    })
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
        let extra_0 = *event.extra.get(0).unwrap_or(&0);
        *current_cpu = extra_0 as u16;

        return Err(Error::from(ErrorKind::Other)); // Do not save this kind of events
    }

    let domain = match code == (code & TRC_SCHED_TO_RUN) {
        true => {
            let extra_0 = *event.extra.get(0).unwrap_or(&0);
            let dom = Domain::from_u32(extra_0);
            cpus_dom.insert(*current_cpu, dom);
            Some(dom)
        }
        false => cpus_dom.get(current_cpu).map(|d| *d),
    }
    .unwrap_or_default();

    Ok(Record {
        cpu: *current_cpu,
        domain,
        event,
    })
}
