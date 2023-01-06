use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// bind the service to this tcp port, default 5555
    #[arg(short, long, default_value = "5555")]
    port: u16,
}

fn main() {
    let args = Args::parse();
    let s = format!("0.0.0.0:{}", args.port)
        .parse::<SocketAddr>()
        .unwrap();
    println!("Listening to {s}");
    let listener = TcpListener::bind(s).unwrap();
    for incoming in listener.incoming() {
        match incoming {
            Ok(incoming) => {
                thread::spawn(|| echo(incoming));
            }

            Err(e) => eprintln!("error {e}"),
        }
    }
}

fn echo(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();

    println!("{peer_addr} - connected!");

    let mut echoed: usize = 0;
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf) {
            Ok(count) => {
                if count == 0 {
                    break;
                } else {
                    // write bytes
                    match stream.write(&buf[0..count]) {
                        Ok(count) => echoed += count,
                        Err(e) => {
                            eprintln!("{peer_addr} - en error occured writing socket {e}");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{peer_addr} - en error occured reading socket {e}");
                break;
            }
        }
    }
    println!("{peer_addr} - connection ended, echoed {echoed} bytes!");
}
