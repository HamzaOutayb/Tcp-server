use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::fmt;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

type Result<T> = std::result::Result<T, ()>;

struct Sensitive<T>(T);

const SAFE_MODE: bool = false;

impl<T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if SAFE_MODE {
            write!(f, "[REDACTED]")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

enum Message {
    ClientConnected,
    ClientDisconnected,
    NewMessage(Vec<u8>),
}

fn server(_receiver: Receiver<Message>) -> Result<()> {
    // Just stubbed out here, you can handle messages from clients
    for message in _receiver {
        match message {
            Message::ClientConnected => println!("INFO: Client connected"),
            Message::ClientDisconnected => println!("INFO: Client disconnected"),
            Message::NewMessage(data) => println!("INFO: Received: {:?}", data),
        }
    }
    Ok(())
}

fn client(mut stream: TcpStream, messages: Sender<Message>) -> Result<()> {
    messages.send(Message::ClientConnected).map_err(|err| {
        eprintln!("ERROR: could not send message to the server thread: {}", err);
        ()
    })?;

    let mut buffer = vec![0u8; 64];

    loop {
        let n = stream.read(&mut buffer).map_err(|err| {
            eprintln!("ERROR: error reading message from user: {}", err);
            let _ = messages.send(Message::ClientDisconnected);
            ()
        })?;

        if n == 0 {
            let _ = messages.send(Message::ClientDisconnected);
            break;
        }

        messages.send(Message::NewMessage(buffer[..n].to_vec())).map_err(|err| {
            eprintln!("ERROR: could not send message to server thread: {}", err);
            ()
        })?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let address = "127.0.0.1:6969";

    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!("ERROR: could not bind {}: {}", address, Sensitive(err));
        ()
    })?;

    println!("INFO: listening on {}", address);
    let (message_sender, message_receiver) = channel();
    thread::spawn(move || {
        if let Err(_) = server(message_receiver) {
            eprintln!("ERROR: server thread crashed");
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let sender_clone = message_sender.clone();
                thread::spawn(move || {
                    if let Err(_) = client(stream, sender_clone) {
                        eprintln!("ERROR: client thread exited with error");
                    }
                });
            }
            Err(err) => {
                eprintln!("ERROR: could not accept connection: {}", err);
            }
        }
    }

    Ok(())
}
