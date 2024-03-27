use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    source: ErrorSource,
}

#[derive(Debug)]
pub enum ErrorSource {
    /// Error originating from I/O operations.
    Io(io::Error),
    /// Error source is none.
    None,
}

impl Error {
    /// Constructs a new `Error` with a custom message and no source.
    pub(crate) fn new<T: fmt::Display>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
            source: ErrorSource::None,
        }
    }

    /// Constructs a new `Error` originating from an I/O error with a custom message.
    pub(crate) fn io_error<T: fmt::Display>(msg: T, source: io::Error) -> Self {
        Self {
            message: msg.to_string(),
            source: ErrorSource::Io(source),
        }
    }
}

impl Error {
    /// Returns a reference to the source of the error.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use xentrace_parser::{Error, error::ErrorSource};
    ///
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    /// let custom_error = Error::io_error("Custom I/O error", io_error);
    ///
    /// assert_eq!(custom_error.error_source(), &ErrorSource::Io(io_error));
    /// ```
    pub fn error_source(&self) -> &ErrorSource {
        &self.source
    }

    /// Returns the OS error that this error represents, if applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use xentrace_parser::{Error, error::ErrorSource};
    ///
    /// let io_error = io::Error::from_raw_os_error(2);
    /// let custom_error = Error::io_error("Custom I/O error", io_error);
    ///
    /// assert_eq!(custom_error.raw_os_error(), Some(2));
    /// ```
    pub fn raw_os_error(&self) -> Option<i32> {
        match &self.source {
            ErrorSource::Io(source) => source.raw_os_error(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;

        match &self.source {
            ErrorSource::Io(source) => {
                f.write_fmt(format_args!(": {}", source))?;
            }
            ErrorSource::None => (),
        }

        Ok(())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            ErrorSource::Io(source) => Some(source),
            ErrorSource::None => None,
        }
    }
}
