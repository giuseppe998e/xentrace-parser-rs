use crate::record::Record;

/// Generated by the [`xentrace_parse`](super::xentrace_parse) function,
/// contains the [records](crate::record::Record) of the parsed XenTrace binary files.
#[derive(Debug, Default)]
pub struct Trace {
    /// Boxed slice of [`Record`](crate::record::Record).
    pub records: Box<[Record]>,
    /// Boxed slice of `u16`.
    /// Contains all the CPUs actually used.
    pub cpus: Box<[u16]>,
}

impl Trace {
    /// Returns the count of CPUs used in the trace file.
    ///
    /// **N.B.** The value is calculated based on the highest id
    /// in the list of CPUs used, the final value may be wrong.
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
        self.cpus.iter().max().map(|v| v + 1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Trace;

    #[test]
    fn cpu_count_test() {
        let trace = Trace {
            records: Box::new([]),
            cpus: Box::new([1, 5, 6, 7, 2, 3, 8, 4]),
        };

        assert_eq!(trace.cpu_count(), 9);
    }
}
