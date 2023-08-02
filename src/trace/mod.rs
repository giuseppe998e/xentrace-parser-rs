use std::{fs::File, io::BufReader, ops::Deref, path::Path};

use crate::record::Record;

mod parse;
use parse::parse_trace;

/// Contains the [Record](crate::record::Record) list of the parsed XenTrace binary file.
/// The trace is truncated to the last readable record, returning no errors.
///
/// # Example
///
/// ```
/// use std::io::Result;
/// use xentrace_parser::Trace;
///
/// fn function() -> Result<()> {
///     let trace = Trace::try_from("/path/to/xentrace.bin")?;
///
///     // let file: File = todo!();
///     // let trace = Trace::from(file);
///
///     // let file: File = todo!();
///     // let bufreader: BufReader<File> = BufReader::new(file);
///     // let trace = Trace::from(bufreader);
///
///     for r in trace.iter() {
///         println!("{:?}", r);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Trace(Box<[Record]>, u16);

impl Trace {
    /// Returns the number of [Records](crate::record::Record) that have been read from the trace file.
    ///
    /// **N.B.** This function returns the same value of `Trace::len()`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::Result;
    /// use xentrace_parser::Trace;
    ///
    /// fn function() -> Result<()> {
    ///     let trace = Trace::try_from("/path/to/xentrace.bin")?;
    ///
    ///     // let file: File = todo!();
    ///     // let trace = Trace::from(file);
    ///
    ///     // let file: File = todo!();
    ///     // let bufreader: BufReader<File> = BufReader::new(file);
    ///     // let trace = Trace::from(bufreader);
    ///
    ///     let record_count = trace.record_count();
    ///     println!("{}", record_count);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn record_count(&self) -> usize {
        self.0.len()
    }

    /// Returns the count of CPUs used in the trace file.
    ///
    /// **N.B.** The value is calculated based on the highest value
    /// in the list of CPUs used, the result may be inaccurate.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::Result;
    /// use xentrace_parser::Trace;
    ///
    /// fn function() -> Result<()> {
    ///     let trace = Trace::try_from("/path/to/xentrace.bin")?;
    ///
    ///     // let file: File = todo!();
    ///     // let trace = Trace::from(file);
    ///
    ///     // let file: File = todo!();
    ///     // let bufreader: BufReader<File> = BufReader::new(file);
    ///     // let trace = Trace::from(bufreader);
    ///
    ///     let cpu_count = trace.cpu_count();
    ///     println!("{}", cpu_count);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn cpu_count(&self) -> u16 {
        self.1
    }
}

impl From<BufReader<File>> for Trace {
    fn from(reader: BufReader<File>) -> Self {
        parse_trace(reader)
    }
}

impl From<File> for Trace {
    fn from(file: File) -> Self {
        let reader = BufReader::new(file);
        parse_trace(reader)
    }
}

impl TryFrom<&Path> for Trace {
    type Error = std::io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        File::open(path).map(Self::from)
    }
}

impl TryFrom<&str> for Trace {
    type Error = std::io::Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let path = Path::new(string);
        Self::try_from(path)
    }
}

impl TryFrom<String> for Trace {
    type Error = std::io::Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let path = Path::new(&string);
        Self::try_from(path)
    }
}

impl Deref for Trace {
    type Target = [Record];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<[Record]> for Trace {
    fn as_ref(&self) -> &[Record] {
        self.0.as_ref()
    }
}
