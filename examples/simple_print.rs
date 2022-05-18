use std::{env, io::Result};
use xentrace_parser::{
    record::{Domain, DomainType /*, Record*/, Event /*, EventCode*/},
    xentrace_parse, Trace,
};

fn main() -> Result<()> {
    let trace_file = {
        let current_dir = env::current_dir().unwrap();
        format!("{}/{}", current_dir.to_str().unwrap(), "examples/trace.xen")
    };

    let trace: Trace = xentrace_parse(&trace_file)?;
    let records = &trace.records;

    for r in records {
        let _cpu: u16 = r.cpu; // Host CPU
        let domain: Domain = r.domain;
        let _dom_type: DomainType = domain.type_;
        let _event: Event = r.event.clone();

        println!("{:?}", r);
    }

    println!(); // Blank

    let rec_count = records.len();
    let cpu_count = trace.cpu_count();

    println!("Records count: {:?}", rec_count);
    println!("Host CPU count:  {:?}", cpu_count);

    Ok(())
}
