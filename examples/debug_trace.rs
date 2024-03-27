use std::{
    fs,
    io::{self, Write},
};

use xentrace_parser::Trace;

fn main() {
    let path = fs::canonicalize("./examples/xentrace.bin").unwrap();
    let trace = Trace::from_file(path).unwrap();
    debug_trace(&trace);
}

fn debug_trace(trace: &Trace) {
    let mut stdout = io::stdout().lock();

    // Print records
    for r in trace.iter() {
        let _ = writeln!(&mut stdout, "{:?}", r);
    }

    // Blank line
    let _ = writeln!(&mut stdout);

    // Print record count
    let rec_count = trace.record_count(); // or trace.len()
    let _ = writeln!(&mut stdout, "Record count: {}", rec_count);

    // Print host's CPU count
    let cpu_count = trace.cpu_count();
    let _ = writeln!(&mut stdout, "Host CPU count:  {}", cpu_count);
}
