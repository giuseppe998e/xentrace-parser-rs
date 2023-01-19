use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Result},
    rc::Rc,
};

use crate::{
    record::{Domain, Event, EventCode, Record, EVENT_EXTRA_MAXLEN},
    util::bufreader::ReadNumber,
};

const TRC_TRACE_CPU_CHANGE: u32 = 0x0001F003;
const TRC_SCHED_TO_RUN: u32 = 0x00021F0F;

struct ParseData {
    records: Vec<Record>,
    domains: HashMap<u16, Rc<Domain>>,
    last_tsc: u64,
    current_cpu: u16,
}

pub(super) fn parse_trace(mut reader: BufReader<File>) -> super::Trace {
    let mut parse_data = ParseData {
        records: Vec::with_capacity(u16::MAX as usize),
        domains: HashMap::with_capacity(u16::BITS as usize),
        last_tsc: 0,
        current_cpu: 0,
    };

    while let Some(record) = parse_record(&mut reader, &mut parse_data) {
        if record.event.code == TRC_TRACE_CPU_CHANGE {
            parse_data.current_cpu = record.event.extra[0].unwrap_or(0) as u16;
            continue;
        }

        parse_data.records.push(record);
    }

    let cpu_count = parse_data.domains.len() as u16;
    let records = {
        parse_data.records.sort();
        parse_data.records.into_boxed_slice()
    };

    super::Trace(records, cpu_count)
}

fn parse_record(buf: &mut BufReader<File>, parse_data: &mut ParseData) -> Option<Record> {
    let event = parse_event(buf, parse_data).ok()?;

    let domain = match event.code == (event.code & TRC_SCHED_TO_RUN) {
        true => {
            let extra_0 = event.extra[0].unwrap_or(0);
            let domain = Rc::new(Domain::from(extra_0));
            parse_data
                .domains
                .insert(parse_data.current_cpu, Rc::clone(&domain));
            domain
        }
        false => parse_data
            .domains
            .get(&parse_data.current_cpu)
            .map(Rc::clone)
            .unwrap_or_default(),
    };

    Some(Record {
        cpu: parse_data.current_cpu,
        domain,
        event,
    })
}

fn parse_event(buf: &mut BufReader<File>, parse_data: &mut ParseData) -> Result<Event> {
    let header = buf.read_ne_u32()?;

    // EventCode
    let code = {
        let code = header & 0x0FFFFFFF;
        EventCode::from(code)
    };

    // Timestamp (CPU cycle counter)
    let tsc = {
        let include_tsc = (header & (1 << 31)) > 0;
        if include_tsc {
            parse_data.last_tsc = buf.read_ne_u64()?;
        }

        parse_data.last_tsc
    };

    // Extra list
    let extra = {
        let extra_count = ((header >> 28) as usize) & EVENT_EXTRA_MAXLEN;
        let mut extra_list = [None; EVENT_EXTRA_MAXLEN];

        if extra_count > 0 {
            for extra in extra_list.iter_mut().take(extra_count) {
                *extra = buf.read_ne_u32().map(Some)?;
            }
        }

        extra_list
    };

    Ok(Event { code, tsc, extra })
}
