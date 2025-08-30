use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    fine!("Request: {http_request:#?}");

    exiting!();
}
