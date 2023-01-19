use std::{
    env,
    io::{Result, Write},
};
use xentrace_parser::Trace;

fn main() -> Result<()> {
    let trace = {
        let mut path = env::current_dir().unwrap();
        path.push("examples/trace.xen");
        Trace::try_from(path.as_path())?
    };

    {
        let mut stdout = std::io::stdout().lock();
        for r in trace.iter() {
            let _ = writeln!(&mut stdout, "{:?}", r);
        }

        let _ = writeln!(&mut stdout); // Blank
    }

    let rec_count = trace.record_count();
    let cpu_count = trace.cpu_count();

    println!("Records count: {:?}", rec_count);
    println!("Host CPU count:  {:?}", cpu_count);

    Ok(())
}
