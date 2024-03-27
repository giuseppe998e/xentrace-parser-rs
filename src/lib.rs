#![deny(unsafe_code)]

mod error;
pub mod record;
mod trace;
mod util;

pub use self::{
    error::{Error, Result},
    trace::Trace,
};
