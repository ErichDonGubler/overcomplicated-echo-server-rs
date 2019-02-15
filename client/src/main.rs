use {
    shared::{BufDisplay, BUF_SIZE},
    std::{
        io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write},
        net::{Shutdown, SocketAddr, TcpStream},
    },
    structopt::StructOpt,
};

#[derive(StructOpt)]
pub struct Cli {
    message: String,
    server: SocketAddr,
}

fn main() -> IoResult<()> {
    let Cli { message, server } = Cli::from_args();

    if message.len() > BUF_SIZE {
        // TODO: This definitely should be documented before landing in `master`.
        return Err(IoError::new(
            IoErrorKind::InvalidData,
            "input message larger than allowed",
        ));
    }

    let mut stream = TcpStream::connect(server)?;

    stream.write_all(&message.as_bytes()[..message.len()])?;
    println!("Message sent.");
    stream.shutdown(Shutdown::Write)?;

    let mut buf = [0; BUF_SIZE]; // OPT: MAYBE use an ArrayVec to not default-init stuff
    println!("Awaiting echo...");
    let bytes_read = stream.read(&mut buf)?;

    if message.as_bytes() != &buf[..bytes_read] {
        return Err(IoError::new(
            IoErrorKind::Other,
            format!(
                "message provided not echoed, got something else; received {:?}",
                BufDisplay(&buf[..bytes_read])
            ),
        ));
    }

    println!("Success!");

    Ok(())
}
