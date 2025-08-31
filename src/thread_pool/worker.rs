//
// File Name:    worker.rs
// Directory:    src/thread_pool
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
//! # Worker mod
//!

use super::Job;
use crate::*;
use std::{
    fmt,
    sync::{Arc, Mutex, mpsc::Receiver},
    thread::{self, Thread},
};

const_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .set_level(DEBUG_LEVEL)
        .build()
});

#[derive(Debug)]
pub(super) struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    #[logger]
    pub(super) fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Job>>>,
    ) -> Result<Self, std::io::Error> {
        entering!("id: {id}, receiver: {receiver:?}");

        let builder = thread::Builder::new();

        let rtn = match builder.spawn(move || {
            let mut log = Logger::builder(module_path!())
                .set_fn_name("spawn")
                .add_console_handler()
                .set_level(DEBUG_LEVEL)
                .build();

            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        log.finest(&format!("Worker {id} got a job; executing."));
                        job();
                    }
                    Err(_) => {
                        log.finest(&format!("Worker {id} disconnected; shutting down."));
                        break;
                    }
                };
            }
        }) {
            Ok(thread) => Ok(Worker { id, thread:Some(thread) }),
            Err(e) => Err(e),
        };

        exiting!("rtn: {rtn:?}");

        rtn
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {:?}", self.id, self.thread)
    }
}
