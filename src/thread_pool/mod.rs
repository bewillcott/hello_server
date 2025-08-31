//
// File Name:    mod.rs
// Directory:    src/thread_pool
// Project Name: hello
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
//! # ThreadPool mod
//!

mod pool_creation_error;
mod worker;

pub use pool_creation_error::PoolCreationError;
use std::{
    fmt,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
};
use worker::Worker;
use crate::*;

type Job = Box<dyn FnOnce() + Send + 'static>;

const_logger!({
    Logger::builder(module_path!())
        .add_console_handler()
        .set_level(DEBUG_LEVEL)
        .build()
});

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    ///
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// ## Panics
    ///
    /// The `new` function will panic if the size is zero.
    ///
    #[logger]
    pub fn new(size: usize) -> Self {
        entering!("size: {size}");

        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let rtn = setup_thread_pool(size, sender, receiver);

        exiting!("rtn: {rtn}");

        rtn
    }

    ///
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// ## Returns
    ///
    /// `Ok` with new `ThreadPool`, or `Err` with `PoolCreationError` if
    /// `size` is `0`.
    ///
    #[logger]
    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        entering!("size: {size}");

        let rtn = if size == 0 {
            Err(PoolCreationError::new())
        } else {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            Ok(setup_thread_pool(size, sender, receiver))
        };

        exiting!("rtn: {rtn:?}");

        rtn
    }

    #[logger]
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        entering!();

        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();

        exiting!();
    }
}

impl fmt::Display for ThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();

        for worker in &self.workers {
            buf.push_str(&worker.to_string());
        }

        buf.fmt(f)
    }
}

impl Drop for ThreadPool {
    #[logger]
    fn drop(&mut self) {
        entering!();

        drop(self.sender.take());

        for worker in &mut self.workers{
            finest!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }        }
    }
}

#[logger]
fn setup_thread_pool(
    size: usize,
    sender: Sender<Job>,
    receiver: Arc<Mutex<Receiver<Job>>>,
) -> ThreadPool {
    entering!("size: {size}, sender: {sender:?}");

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        if let Ok(worker) = Worker::new(id, Arc::clone(&receiver)) {
            workers.push(worker);
        } else {
            unsafe {
                workers.set_len(id);
            }

            break;
        }
    }

    let rtn = ThreadPool { workers, sender: Some(sender) };

    exiting!("rtn: {rtn}");

    rtn
}
