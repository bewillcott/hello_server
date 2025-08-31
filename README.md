<!-- markdownlint-disable-file MD033 -->

# Hello Server

This project is heavily derived from the "Hello" web server developed in
[Chapter 21][21]: "Final Project: Building a Multithreaded Web Server", in
**The Rust Programming Language** book.

I am making it available here as a further example of the possible uses for my other project:
<a href="https://github.com/bewillcott/flogging" target="_blank">FLogging</a> (<a href="https://crates.io/crates/flogging" target="_blank">crate</a>) (<a href="https://docs.rs/flogging/latest/flogging" target="_blank">api</a>) (<a href="https://bewillcott.github.io/flogging" target="_blank">guide</a>).

This project uses both the **macros** and the **methods** of FLogging.

The most notable use, is in: `hello_server::thread_pool::worker::new()`, where
the **macros** are used for the `Worker`'s methods, and the **methods** are used
in the _closure_ for the `builder.spawn()` method.

If you download and run this project, you will see enough information to understand
what is going on with the server.

## Test run

1. Clone this repository, or download the zip file and unpack it.
2. cd into the root directory of the project, and run it: `cargo run`.
3. Open your web browser to a new tab for each of:\
   i. `http://127.0.0.1:7878/`\
   ii. `http://127.0.0.1:7878/test`\
   iii. `http://127.0.0.1:7878/sleep`\
   iv. `http://127.0.0.1:7878/shutdown`

You should see something like:

<details>
<summary>Output (click to view)</summary>

```text
hello_server::thread_pool->new [FINER  ] Entry: (size: 4)
hello_server::thread_pool->setup_thread_pool [FINER  ] Entry: (size: 4, sender: Sender { .. })
hello_server::thread_pool::worker->new [FINER  ] Entry: (id: 0, receiver: Mutex { data: Receiver { .. }, poisoned: false, .. })
hello_server::thread_pool::worker->new [FINER  ] Return: (rtn: Ok(Worker { id: 0, thread: Some(JoinHandle { .. }) }))
hello_server::thread_pool::worker->new [FINER  ] Entry: (id: 1, receiver: Mutex { data: Receiver { .. }, poisoned: false, .. })
hello_server::thread_pool::worker->new [FINER  ] Return: (rtn: Ok(Worker { id: 1, thread: Some(JoinHandle { .. }) }))
hello_server::thread_pool::worker->new [FINER  ] Entry: (id: 2, receiver: Mutex { data: <locked>, poisoned: false, .. })
hello_server::thread_pool::worker->new [FINER  ] Return: (rtn: Ok(Worker { id: 2, thread: Some(JoinHandle { .. }) }))
hello_server::thread_pool::worker->new [FINER  ] Entry: (id: 3, receiver: Mutex { data: <locked>, poisoned: false, .. })
hello_server::thread_pool::worker->new [FINER  ] Return: (rtn: Ok(Worker { id: 3, thread: Some(JoinHandle { .. }) }))
hello_server::thread_pool->setup_thread_pool [FINER  ] Return: (rtn: 0: Some(JoinHandle { .. })
1: Some(JoinHandle { .. })
2: Some(JoinHandle { .. })
3: Some(JoinHandle { .. })
)
hello_server::thread_pool->new [FINER  ] Return: (rtn: 0: Some(JoinHandle { .. })
1: Some(JoinHandle { .. })
2: Some(JoinHandle { .. })
3: Some(JoinHandle { .. })
)
Hello Web Server Started (127.0.0.1:7878)
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:34900, fd: 4 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:34900, fd: 4 }, buffer: 0/8192 }
request_line: GET / HTTP/1.1
hello_server::thread_pool->execute [FINER  ] Entry
hello_server::thread_pool->execute [FINER  ] Return
hello_server::thread_pool::worker->spawn [FINEST ] Worker 0 got a job; executing.
hello_server->handle_connection [FINER  ] Entry: (request_line: GET / HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:34900, fd: 4 })
hello_server->handle_connection [FINEST ] response: HTTP/1.1 200 OK
Content-Length: 176

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:34916, fd: 5 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:34916, fd: 5 }, buffer: 0/8192 }
request_line: GET /test HTTP/1.1
hello_server::thread_pool->execute [FINER  ] Entry
hello_server::thread_pool->execute [FINER  ] Return
hello_server::thread_pool::worker->spawn [FINEST ] Worker 1 got a job; executing.
hello_server->handle_connection [FINER  ] Entry: (request_line: GET /test HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:34916, fd: 5 })
hello_server->handle_connection [FINEST ] response: HTTP/1.1 404 NOT FOUND
Content-Length: 206

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45756, fd: 4 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45756, fd: 4 }, buffer: 0/8192 }
request_line: GET /sleep HTTP/1.1
hello_server::thread_pool->execute [FINER  ] Entry
hello_server::thread_pool->execute [FINER  ] Return
hello_server::thread_pool::worker->spawn [FINEST ] Worker 2 got a job; executing.
hello_server->handle_connection [FINER  ] Entry: (request_line: GET /sleep HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45756, fd: 4 })
hello_server->handle_connection [FINEST ] response: HTTP/1.1 200 OK
Content-Length: 190

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello Sleepy!</title>
  </head>
  <body>
    <h1>Hello sleepy!</h1>
    <p>Hi from Rust</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45122, fd: 5 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45122, fd: 5 }, buffer: 0/8192 }
request_line: GET /shutdown HTTP/1.1
hello_server::thread_pool->execute [FINER  ] Entry
hello_server::thread_pool->execute [FINER  ] Return
Shutting down.
hello_server::thread_pool->drop [FINER  ] Entry
hello_server::thread_pool->drop [FINEST ] Shutting down worker 0
hello_server::thread_pool::worker->spawn [FINEST ] Worker 0 disconnected; shutting down.
hello_server::thread_pool::worker->spawn [FINEST ] Worker 3 got a job; executing.
hello_server::thread_pool::worker->spawn [FINEST ] Worker 1 disconnected; shutting down.
hello_server::thread_pool::worker->spawn [FINEST ] Worker 2 disconnected; shutting down.
hello_server->handle_connection [FINER  ] Entry: (request_line: GET /shutdown HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45122, fd: 5 })
hello_server::thread_pool->drop [FINEST ] Shutting down worker 1
hello_server->handle_connection [FINEST ] response: HTTP/1.1 200 OK
Content-Length: 186

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Good bye!</title>
  </head>
  <body>
    <h1>Good bye!</h1>
    <p>See ya from Rust</p>
  </body>
</html>

hello_server::thread_pool->drop [FINEST ] Shutting down worker 2
hello_server::thread_pool->drop [FINEST ] Shutting down worker 3
hello_server->handle_connection [FINER  ] Return
hello_server::thread_pool::worker->spawn [FINEST ] Worker 3 disconnected; shutting down.
```

</details>

## Less run

Now if you edit the file: `src/lib.rs`:

change this:

```rust
pub(crate) const DEBUG_LEVEL: Level = Level::ALL;
                                      ------ ^^^
```

to this:

```rust
pub(crate) const DEBUG_LEVEL: Level = Level::OFF;
                                      ------ ^^^
```

Now restart the server, refresh each of the tabs, and you should see something like:

<details>
<summary>Output (click to view)</summary>

```text
Hello Web Server Started (127.0.0.1:7878)
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37228, fd: 4 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37228, fd: 4 }, buffer: 0/8192 }
request_line: GET / HTTP/1.1
hello_server->handle_connection [FINER  ] Entry: (request_line: GET / HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37228, fd: 4 })
hello_server->handle_connection [FINEST ] response: HTTP/1.1 200 OK
Content-Length: 176

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37230, fd: 5 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37230, fd: 5 }, buffer: 0/8192 }
request_line: GET /test HTTP/1.1
hello_server->handle_connection [FINER  ] Entry: (request_line: GET /test HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37230, fd: 5 })
hello_server->handle_connection [FINEST ] response: HTTP/1.1 404 NOT FOUND
Content-Length: 206

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37240, fd: 4 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37240, fd: 4 }, buffer: 0/8192 }
request_line: GET /sleep HTTP/1.1
hello_server->handle_connection [FINER  ] Entry: (request_line: GET /sleep HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:37240, fd: 4 })
hello_server->main [FINEST ] stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45192, fd: 5 }
hello_server->main [FINEST ] buf_reader: BufReader { reader: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45192, fd: 5 }, buffer: 0/8192 }
request_line: GET /shutdown HTTP/1.1
Shutting down.
hello_server->handle_connection [FINER  ] Entry: (request_line: GET /shutdown HTTP/1.1
stream: TcpStream { addr: 127.0.0.1:7878, peer: 127.0.0.1:45192, fd: 5 })
hello_server->handle_connection [FINEST ] response: HTTP/1.1 200 OK
Content-Length: 186

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Good bye!</title>
  </head>
  <body>
    <h1>Good bye!</h1>
    <p>See ya from Rust</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
hello_server->handle_connection [FINEST ] response: HTTP/1.1 200 OK
Content-Length: 190

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello Sleepy!</title>
  </head>
  <body>
    <h1>Hello sleepy!</h1>
    <p>Hi from Rust</p>
  </body>
</html>

hello_server->handle_connection [FINER  ] Return
```

</details>

## Production run

And finally, we go **production** level by editing the file: `src/main.rs`:

change this:

```rust
        .set_level(Level::ALL)
                   ------ ^^^
```

to this:

```rust
        .set_level(Level::INFO)
                   ------ ^^^^
```

Now restart the server (again), refresh each of the tabs (again),
and you should see something like:

```text
Hello Web Server Started (127.0.0.1:7878)
request_line: GET / HTTP/1.1
request_line: GET /test HTTP/1.1
request_line: GET /sleep HTTP/1.1
request_line: GET /shutdown HTTP/1.1
Shutting down.
```

## README run

As a final proof of concept, let's view a larger html file.

1. Restart the server: `cargo run`
2. Open a new tab to: `http://127.0.0.1:7878/readme`

You should now see the "html" representation of this file.

## Final Word

In-case you are not aware (unlikely), you can shutdown the server
by hitting [ctrl/c] in the terminal window that it (the server) is running in.

I hope this example was of some use to you.

Anyway, have fun, mix and match my FLogging stuff as you need, and have a great
day.

[21]: https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html
