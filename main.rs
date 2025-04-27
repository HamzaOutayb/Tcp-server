use std::net::TcpListener;
use std::result;
use std::io::Write;

type Result<T> = result::Result<T, ()>

fn main() -> Result {
    let address = "127.0.0.1:6969";

    let listener = TcpListener::bind(address).map_err(|err|{
        eprintln!("ERROR: could not bind {address}:{err}")
    })?;
    println!("INFO: listening to {address}")
    for stream in listener.incoming() {
        match stream {
            ok(mut stream) {
                let _ = writeln!(stream, "Hello Friend").map_err(|err|{
                    eprintln("ERROR: could not write message ro user: {err}")
                })
            }
            Err(err) => {
                eprintln("Error: could not accept connection: {err}")
            }
        }
    }
}