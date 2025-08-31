
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

impl fmt::Debug for PoolCreationError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PoolCreationError").finish()
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Can not create a pool with '0' threads.".fmt(f)
    }
}
