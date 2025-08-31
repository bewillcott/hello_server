//
// File Name:    main.rs
// Directory:    src
// Project Name: hello_server
//
// Copyright (C) 2025 Bradley Willcott
//
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This library (crate) is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This library (crate) is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this library (crate).  If not, see <https://www.gnu.org/licenses/>.
//

//!
//! # Hello main
//!

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use flogging::*;
use hello_server::*;

const_logger!({
    Logger::builder(module_path!())
        .add_pconsole_handler()
        .set_level(Level::INFO)
        .build()
});

#[logger]
fn main() {
    let mut shutdown = false;
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    info!(
        "Hello Web Server Started ({})",
        listener.local_addr().unwrap()
    );

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        finest!("stream: {stream:?}");

        let buf_reader = BufReader::new(&stream);
        finest!("buf_reader: {buf_reader:?}");

        let request_line = buf_reader.lines().next().unwrap().unwrap();
        info! {"request_line: {request_line}"};

        if "GET /shutdown HTTP/1.1" == &request_line[..] {
            shutdown = true;
        }

        pool.execute(|| handle_connection(request_line, stream));

        if shutdown {
            break;
        }
    }

    info!("Shutting down.");
}

#[logger]
fn handle_connection(request_line: String, mut stream: TcpStream) {
    entering!("request_line: {request_line}\nstream: {stream:?}");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /readme HTTP/1.1" => ("HTTP/1.1 200 OK", "README.html"),
        "GET /shutdown HTTP/1.1" => ("HTTP/1.1 200 OK", "shutdown.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "hello-sleepy.html")
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
