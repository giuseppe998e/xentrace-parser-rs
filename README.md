# XenTrace binary data parser (RUST LANG) [![GitHub Latest Tag](https://img.shields.io/github/v/tag/giuseppe998e/xentrace-parser-rs?style=flat-square)](https://github.com/giuseppe998e/xentrace-parser-rs/tags) [![Crates.io Downloads](https://img.shields.io/crates/d/xentrace-parser?style=flat-square)](https://crates.io/crates/xentrace-parser)

This library parses XenTrace binary files by producing a list of events sorted by their TSC.  
This is the Rust lang version of a [project](https://github.com/giuseppe998e/xentrace-parser) made for the final three-year degree exam at the University of Turin.

## Dependencies

- `rust` (v1.56.1+)

## Usage

```rust
use xentrace_parser::{
    record::{Domain, DomainKind, Event, Record/*, EventCode*/},
    xentrace_parse, Trace,
};

fn main() -> Result<()> {
    let trace: Trace = xentrace_parse("/path/to/trace.xen.dat")?;

    for record in trace.iter() {
        let _cpu: u16 = record.cpu(); // Host CPU
        let domain: &Domain = record.domain();
        let _dom_kind: DomainKind = domain.kind();
        let _event: &Event = record.event();

        println!("{:?}", r);
    }

    println!(); // Blank

    let rec_count = trace.record_count();
    let cpu_count = trace.cpu_count();

    println!("Records count: {:?}", rec_count);
    println!("Host CPU count:  {:?}", cpu_count);

    Ok(())
}
```

> This example could be started from the repo's root directory with: `cargo run --example simple_print`

## License

This library is released under the `GNU Lesser General Public License v2.1 (or later)`.
