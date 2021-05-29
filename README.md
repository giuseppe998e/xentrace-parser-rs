# XenTrace binary data parser (RUST LANG) [![](https://img.shields.io/github/v/tag/giuseppe998e/xentrace-parser-rs?style=flat-square)](https://github.com/giuseppe998e/xentrace-parser-rs/tags)

This library parses XenTrace binary files by producing a list of events sorted by their TSC.  
This is the Rust lang version of a [project](https://github.com/giuseppe998e/xentrace-parser) made for the final three-year degree exam at the University of Turin.  

## Usage
```rust
use xentrace_parser::{Parser, /*Record,*/ Event, Domain, DomainType};

fn main() -> std::io::Result<()> {
    let parser = Parser::new("/home/giuseppe/Downloads/trace_xen.bin")?;
    let records = parser.get_records(); // Vec<Record>

    for r in records {
        let _cpu: u8 = r.get_cpu(); // Host CPU
        let _domain: Domain = r.get_domain();
        let _domType: DomainType = _domain.get_type();
        let _event: Event = r.get_event();

        println!("{:?}", r);
    }

    println!(); // Blank

    let rec_count = records.len();
    let cpu_count = parser.cpu_count();

    println!("Records count: {:?}", rec_count);
    println!("Host CPU count:  {:?}", cpu_count);

    Ok(())
}
```

## License
This library is released under the `GNU Lesser General Public License v2.1 (or later)`.  