use {
    display_derive::Display,
    std::{
        fmt::{Debug, Formatter, Result as FmtResult},
        io::{Error as IoError, Read},
        net::TcpStream,
    },
};

pub const BUF_SIZE: usize = 128;

pub type Buffer = [u8; BUF_SIZE];

pub struct BufDisplay<'b>(pub &'b [u8]);

impl Debug for BufDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self(arr) = self;
        f.debug_list().entries(arr.iter()).finish()
    }
}

#[derive(Display)]
pub enum StreamReadError {
    #[display(fmt = "unable to read message from stream: {}", _0)]
    Io(IoError),
    #[display(
        fmt = "got message that's larger than allowed, assuming garbage and letting this drop"
    )]
    MessageTooBig,
}

pub fn read_validated_message_from_stream(
    buf: &mut Buffer,
    stream: &mut TcpStream,
) -> Result<usize, StreamReadError> {
    use self::StreamReadError::*;

    let bytes_read = stream.read(buf).map_err(Io)?;

    if stream
        .peek(&mut [0; 1])
        .map(|read| read != 0)
        .unwrap_or(false)
    {
        // @TODO: This definitely should be documented before landing in production.
        Err(MessageTooBig)
    } else {
        Ok(bytes_read)
    }
}
