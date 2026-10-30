---
title: A TCP echo server
track: Systems
summary: Listen on a socket, serve each client on its own thread, handle disconnects and errors, and test it end to end.
concepts: TcpListener, TcpStream, Read/Write, io::Result, threads, port 0 in tests
---
## What you'll build
A server that echoes back whatever each connected client sends, and a test that connects to it.

## Run it
```bash
cargo run -p lesson-10-tcp-echo
# in another terminal:
nc 127.0.0.1 7878
cargo test -p lesson-10-tcp-echo
```

## How it works
- `TcpListener::bind` claims a port; `incoming()` yields a `TcpStream` per client.
- `read` returning **0 bytes** means the client hung up; the loop must stop there.
- `write_all` keeps writing until every byte is sent (plain `write` may send only part).
- Errors are returned as `io::Result` and logged per client, so one bad client can't crash the server.
- The test binds to port `0`, so the operating system picks a free port.

## Where this came from
The original `Cyber/TCPServer/Main.rs` looped forever when a client disconnected (it kept echoing zero bytes), used `write` instead of `write_all`, and unwrapped every result. All three are fixed here.

## Try this
1. Review, lesson 05: give `handle_client` its own error enum instead of `io::Error`, with a `Display` message per case.
2. Uppercase the reply instead of echoing it.
3. Close idle clients after 30 seconds (`set_read_timeout`).
4. Rewrite it with `tokio` and async tasks, and compare.
