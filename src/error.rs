use std::{error, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<Box<dyn error::Error + 'static>>,
}

impl Error {
    pub(crate) fn new<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self {
            message: msg.to_string(),
            source: None,
        }
    }

    pub(crate) fn new_source<T, E>(msg: T, source: E) -> Self
    where
        T: fmt::Display,
        E: error::Error + 'static,
    {
        Self {
            message: msg.to_string(),
            source: Some(Box::from(source)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)?;

        if let Some(source) = self.source.as_ref() {
            f.write_fmt(format_args!(": {}", source))?;
        }

        Ok(())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_deref()
    }
}
