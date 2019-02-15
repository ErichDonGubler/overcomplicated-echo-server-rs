use std::fmt::{Debug, Formatter, Result as FmtResult};

pub const BUF_SIZE: usize = 128;

pub struct BufDisplay<'b>(pub &'b [u8; BUF_SIZE]);

impl Debug for BufDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self(arr) = self;
        f.debug_list().entries(arr.iter()).finish()
    }
}
