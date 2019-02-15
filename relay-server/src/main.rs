use {
    shared::{read_validated_message_from_stream, BufDisplay, BUF_SIZE},
    std::{
        io::{Result as IoResult, Write},
        net::{SocketAddr, TcpStream},
    },
    structopt::StructOpt,
};

#[derive(StructOpt)]
pub struct Cli {
    relay_addr: SocketAddr,
}

fn main() -> IoResult<()> {
    let Cli { relay_addr } = Cli::from_args();

    loop {
        let mut stream = TcpStream::connect(relay_addr)?;
        println!("Connected to relay. Waiting for message...");

        let mut buf = [0; BUF_SIZE]; // OPT: MAYBE use an ArrayVec to not default-init stuff

        match read_validated_message_from_stream(&mut buf, &mut stream) {
            Ok(bytes_read) => {
                println!("Got message {:?} ", BufDisplay(&buf[..bytes_read]));
                stream.write_all(&buf[..bytes_read])?;
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        println!("Disconnecting from relay...");
    }
}
