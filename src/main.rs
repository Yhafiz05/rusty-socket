use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::net::{Ipv4Addr,SocketAddrV4};
use std::io::{BufRead, BufReader, BufWriter, Write};

const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 8080;
fn main() {
    let tcpListener = TcpListener::bind(SocketAddrV4::new(ADDR,PORT)).unwrap();
    println!("Listening on port {}", PORT);
    for stream in tcpListener.incoming() {
        // Error handling
        match stream {
            Ok(stream) => {
                // move the ownership of "stream" to the thread
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                eprint!("Connection failed cause by {}", e);
            }
        }
    }

}

fn handle_client(mut stream: TcpStream) {
    let mut readline = BufReader::new(&stream);
    let mut writeline  = BufWriter::new(&stream);
    writeln!(writeline, "welcome to the echo server !!!").unwrap();
    writeln!(writeline, "For exiting press enter").unwrap();
    writeline.flush().unwrap();

    let mut line = String::new();
    while match readline.read_line(&mut line){
        Ok(1) =>{
            false
        },
        Ok(n) => {
            writeln!(writeline, "{}", line.trim_end()).unwrap();
            writeline.flush().unwrap();
            line.clear();
            true
        },
        Err(e) => {
            eprint!("AN error occurred reading from stream: {}", e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}

    stream.shutdown(Shutdown::Both).unwrap()
}