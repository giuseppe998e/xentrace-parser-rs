use std::ops::Deref;

use crate::record::Record;

/// Generated by the [`xentrace_parse`](super::xentrace_parse) function,
/// contains the [Record](crate::record::Record) list of the parsed XenTrace binary files.
///
/// # Examples
///
/// ```
/// use std::io::Result;
/// use xentrace_parser::{xentrace_parse, Trace};
///
/// fn function() -> Result<()> {
///     let trace: Trace = xentrace_parse("/path/to/trace.xen.dat")?;
///
///     for r in trace.iter() {
///         println!("{:?}", r);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Default)]
pub struct Trace(pub(crate) Box<[Record]>, pub(crate) u16);

impl Trace {
    /// Returns the number of [Records](crate::record::Record) that have been read from the trace file.
    ///
    /// **N.B.** This function returns the same value of `Trace::len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Result;
    /// use xentrace_parser::{xentrace_parse, Trace};
    ///
    /// fn function() -> Result<()> {
    ///     let trace: Trace = xentrace_parse("/path/to/xentrace.bin")?;
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
    /// # Examples
    ///
    /// ```
    /// use std::io::Result;
    /// use xentrace_parser::{xentrace_parse, Trace};
    ///
    /// fn function() -> Result<()> {
    ///     let trace: Trace = xentrace_parse("/path/to/xentrace.bin")?;
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

impl Deref for Trace {
    type Target = [Record];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
