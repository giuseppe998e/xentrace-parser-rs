use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

pub(crate) trait ReadNumber {
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
}

impl ReadNumber for BufReader<File> {
    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_ne_bytes(buf)) // host endian because of XenTrace
    }

    #[inline]
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf)) // host endian because of XenTrace
    }
}
