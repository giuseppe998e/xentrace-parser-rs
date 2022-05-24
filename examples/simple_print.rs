use std::{env, io::Result};
use xentrace_parser::{xentrace_parse, Trace};

fn main() -> Result<()> {
    let trace_file = {
        let current_dir = env::current_dir().unwrap();
        format!("{}/{}", current_dir.to_str().unwrap(), "examples/trace.xen")
    };

    let trace: Trace = xentrace_parse(&trace_file)?;

    for r in trace.iter() {
        println!("{:?}", r);
    }

    println!(); // Blank

    let rec_count = trace.record_count();
    let cpu_count = trace.cpu_count();

    println!("Records count: {:?}", rec_count);
    println!("Host CPU count:  {:?}", cpu_count);

    Ok(())
}
