use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use flogging::*;

const_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .set_level(Level::ALL)
        .build()
});

#[logger]
fn main() {
    entering!();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    finer!("listener: {listener:?}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        finest!("stream: {stream:?}");

        handle_connection(stream);
    }
}

#[logger]
fn handle_connection(mut stream: TcpStream) {
    entering!("stream: {stream:?}");

    let buf_reader = BufReader::new(&stream);
    finest!("buf_reader: {buf_reader:?}");

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    finest! {"request_line: {request_line}"};

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    finest!("response: {response}");
    stream.write_all(response.as_bytes()).unwrap();

    exiting!();
}
