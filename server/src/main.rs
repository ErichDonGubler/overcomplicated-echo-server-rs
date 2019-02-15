use {
    shared::{read_validated_message_from_stream, BufDisplay, BUF_SIZE},
    std::{
        io::{Result as IoResult, Write},
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

        match read_validated_message_from_stream(&mut buf, &mut stream) {
            Ok(bytes_read) => {
                println!("Got message {:?} ", BufDisplay(&buf[..bytes_read]));
                stream.write_all(&buf[..bytes_read])?;
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
