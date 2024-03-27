pub(crate) use self::reader::IoReadUtil;

mod reader {
    use std::io;

    pub trait IoReadUtil {
        fn read_ne_u32(&mut self) -> io::Result<u32>;
        fn read_ne_u64(&mut self) -> io::Result<u64>;
    }

    impl<R: io::Read> IoReadUtil for R {
        #[inline]
        fn read_ne_u32(&mut self) -> io::Result<u32> {
            let mut buf = [0u8; 4];
            self.read_exact(&mut buf)?;
            Ok(u32::from_ne_bytes(buf)) // host endian because of XenTrace
        }

        #[inline]
        fn read_ne_u64(&mut self) -> io::Result<u64> {
            let mut buf = [0u8; 8];
            self.read_exact(&mut buf)?;
            Ok(u64::from_ne_bytes(buf)) // host endian because of XenTrace
        }
    }
}
