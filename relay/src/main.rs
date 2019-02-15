use {
    shared::{read_validated_message_from_stream, BufDisplay, BUF_SIZE},
    std::{
        io::{Result as IoResult, Write},
        net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpListener},
    },
    structopt::StructOpt,
};

#[derive(StructOpt)]
pub struct Cli {
    incoming_port: u16,
    outgoing_port: u16,
}

fn main() -> IoResult<()> {
    let Cli {
        incoming_port,
        outgoing_port,
    } = Cli::from_args();

    let server_listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        outgoing_port,
    )))?;

    let client_listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        incoming_port,
    )))?;

    loop {
        println!("Waiting for connection from server...");
        let (mut server_stream, addr) = server_listener.accept()?;
        println!("Got connection from server at {}", addr);

        println!("Waiting for connection from client...");
        let (mut client_stream, addr) = client_listener.accept()?;
        println!("Got connection from server at {}", addr);

        let mut client_buf = [0; BUF_SIZE]; // OPT: MAYBE use an ArrayVec to not default-init stuff
        let mut server_buf = [0; BUF_SIZE]; // OPT: MAYBE use an ArrayVec to not default-init stuff

        match read_validated_message_from_stream(&mut client_buf, &mut client_stream) {
            Ok(bytes_read) => {
                println!(
                    "Got message from client, forwarding to server: {:?}",
                    BufDisplay(&client_buf[..bytes_read])
                );
                server_stream.write_all(&client_buf[..bytes_read])?;
                server_stream.shutdown(Shutdown::Write)?;

                match read_validated_message_from_stream(&mut server_buf, &mut server_stream) {
                    Ok(bytes_read) => {
                        println!(
                            "Got echoed message from server, forwarding back to client: {:?}",
                            BufDisplay(&client_buf[..bytes_read])
                        );
                        client_stream.write_all(&server_buf[..bytes_read])?;
                        client_stream.shutdown(Shutdown::Write)?;
                    }
                    Err(e) => eprintln!("Error reading echoed message from server: {}", e),
                }
            }
            Err(e) => eprintln!("Error reading message from client: {}", e),
        }
    }
}
