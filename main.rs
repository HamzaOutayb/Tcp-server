use std::net::{TcpListener, TcpStream};
use std::result;
use std::io::Read;
use std::fmt;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

type Result<T> = result::Result<T, ()>;

struct Sensitive<T>(T);

const SAFE_MODE: bool = false;

impl<T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        } else {
            inner.fmt(f)
        }
    }
}

enum Message {
    ClientConnected,
    ClientDisconnected,
    NewMessage(Vec<u8>),
}

fn server(_message: Receiver<Message>) -> Result<()> {
       todo!("{:?}", ())
}

fn client(mut stream: TcpStream, messages: Sender<Message>) -> Result<()> {
    messages.send(Message::ClientConnected).map_err(|err| {
        eprintln!("ERROR: could not send message to the server thread: {err}");
    })?;
    let mut buffer = Vec::new();
    buffer.resize(64, 0);
    loop {
       let n = stream.read(&mut buffer).map_err(|err| {
        eprintln!("ERROR: error reading message from user {}", err);
        let _ = messages.send(Message::ClientDisconnected);
       });
       messages.send(Message::NewMessage(buffer[0..n?])).map_err(|err| {
        eprintln!("ERROR: could not send message to server thread: {}", err);
       })?;
    }
}

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";

    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {}: {}", address, Sensitive(err));
    })?;

    println!("INFO: listening to {}", address);
    let (message_sender, message_reciever) = channel();
    thread::spawn(move || server(message_reciever));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let message_sender = message_sender.clone();
                thread::spawn(|| {|| client(stream, message_sender)});
            }
            Err(err) => {
                eprintln!("ERROR: could not accept connection: {}", err);
            }
        };
    }

    Ok(())
}
