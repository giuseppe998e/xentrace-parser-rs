# XenTrace binary data parser (RUST LANG) [![GitHub Latest Tag](https://img.shields.io/github/v/tag/giuseppe998e/xentrace-parser-rs?style=flat-square)](https://github.com/giuseppe998e/xentrace-parser-rs/tags) [![Crates.io Downloads](https://img.shields.io/crates/d/xentrace-parser?style=flat-square)](https://crates.io/crates/xentrace-parser)

This library parses XenTrace binary files by producing a list of event records sorted by their TSC.

> This is the Rust lang version of a [project](https://github.com/giuseppe998e/xentrace-parser) made for the final three-year degree exam at the University of Turin.

## Dependencies

- `rust` (v1.65+)

## Usage

```rust 
use xentrace_parser::{Result, Trace};

fn main() -> Result<()> {
    // The trace is truncated to the last readable record, returning no errors.
    let trace = Trace::from_file("/path/to/xentrace.bin")?;

    // Alternatively, you can create a trace from a bytes slice:
    // let bytes: Vec<u8> = vec![/* byte data */];
    // let trace = Trace::from_bytes(&bytes)?;

    // Alternatively, you can create a trace from a reader:
    // let file = std::fs::File::open("/path/to/xentrace.bin")?;
    // let bufreader = std::io::BufReader::new(file);
    // let trace = Trace::from_reader(bufreader);

    for record in trace.iter() {
        println!("{:?}", record);
    }

    Ok(())
}
```

> An example debug can be started from the root directory with: `cargo run --example debug_trace` (only available on GitHub sources).

## License

This library is released under the [`GNU Lesser General Public License v2.1 (or later)`](./LICENSE).
