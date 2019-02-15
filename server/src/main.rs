use {
    shared::{BufDisplay, BUF_SIZE},
    std::{
        io::{Read, Result as IoResult, Write},
        net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
    },
    structopt::StructOpt,
};

#[derive(StructOpt)]
pub struct Cli {
    port: u16,
}

fn main() -> IoResult<()> {
    let Cli { port } = Cli::from_args();

    let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        port,
    )))?;

    loop {
        println!("Waiting for connection...");
        let (mut stream, addr) = listener.accept()?;
        println!("Got connection from address {}", addr);
        let mut buf = [0; BUF_SIZE]; // OPT: MAYBE use an ArrayVec to not default-init stuff

        let bytes_read = stream.read(&mut buf)?;

        if stream.peek(&mut [0; 1]).map(|read| read != 0).unwrap_or(false) {
            // TODO: This definitely should be documented before landing in `master`.
            eprintln!("Error: got message that's larger than allowed, assuming garbage and letting this drop");
        } else {
            println!("Got message {:?} ", BufDisplay(&buf));
            stream.write(&buf[..bytes_read])?;
        }
    }
}
