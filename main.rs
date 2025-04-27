use std::net::TcpListener;
use std::result;
use std::io::Write;

type Result<T> = result::Result<T, ()>;

struct Sensitive<T> {
    inner: T
}

impl<T: Display> Display for Sensitive<T>

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";

    // Bind to the address
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {address}: {err}");
    })?;

    // Print a message when it's listening
    println!("INFO: listening to {address}");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = writeln!(stream, "Hello Friend").map_err(|err| {
                    eprintln!("ERROR: could not write message to user: {err}");
                });
            }
            Err(err) => {
                eprintln!("ERROR: could not accept connection: {err}");
            }
        }
    }

    Ok(())
}