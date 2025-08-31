//
// File Name:    pool_creation_error.rs
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
//! # PoolCreationError mod
//!

use std::fmt;

pub struct PoolCreationError;

impl PoolCreationError {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PoolCreationError {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PoolCreationError").finish()
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Can not create a pool with '0' threads.".fmt(f)
    }
}
