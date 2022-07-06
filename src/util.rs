use std::{
    fs::File,
    io::{Read, Result},
};

#[inline]
pub(crate) fn read_u32(file: &mut File) -> Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_ne_bytes(buf)) // host-endian because of XenTrace
}

#[inline]
pub(crate) fn read_u64(file: &mut File) -> Result<u64> {
    let mut buf = [0u8; 8];
    file.read_exact(&mut buf)?;
    Ok(u64::from_ne_bytes(buf)) // host-endian because of XenTrace
}
