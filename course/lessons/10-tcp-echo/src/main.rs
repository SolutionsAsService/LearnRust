//! Lesson 10: a TCP echo server, one thread per client.
//! Grown from the repository's original `Cyber/TCPServer/Main.rs`.

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Echo everything a client sends until it hangs up.
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer = stream.peer_addr()?;
    let mut buffer = [0u8; 1024];
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            println!("{peer} disconnected");
            return Ok(()); // 0 bytes means the client closed the connection
        }
        stream.write_all(&buffer[..n])?;
        println!("{peer}: echoed {n} bytes");
    }
}

/// Accept clients forever, each on its own thread.
fn serve(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("client error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept failed: {e}"),
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("echo server on {}  (try: nc 127.0.0.1 7878)", listener.local_addr()?);
    serve(listener);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echoes_what_it_receives() {
        // Port 0 asks the OS for any free port, so tests never collide.
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || serve(listener));

        let mut client = TcpStream::connect(addr).unwrap();
        client.write_all(b"hello rust").unwrap();
        let mut reply = [0u8; 10];
        client.read_exact(&mut reply).unwrap();
        assert_eq!(&reply, b"hello rust");
    }
}
