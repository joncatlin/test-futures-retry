use futures_retry::{ErrorHandler, RetryPolicy};
use std::io;
use std::time::Duration;
use std::cmp;

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, JonError>;

pub const MAX_ATTEMPTS: usize = 5;

#[derive(Debug)]
pub enum JonError {
    IoError(io::Error),
    General,
}

impl From<io::Error> for JonError {
    fn from(error: io::Error) -> Self {
        info!("In impl From<io::Error> for JonError. error={:?}", error);
        JonError::IoError(error)
    }
}

impl From<()> for JonError {
    fn from(error: ()) -> Self {
        info!("In impl From<()> for JonError. error={:?}", error);
        JonError::General
    }
}

impl From<(JonError, usize)> for JonError {
    fn from((error, attempts): (JonError, usize)) -> Self {
        info!("In impl From<(JonError, usize)> for JonError. error={:?} attempts={}", error, attempts);
        JonError::General
    }
    // fn from(_: T) -> Self { unimplemented!() }
}

impl fmt::Display for JonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred in the Application!")
    }
}

//******************************** CUSTOM HANDLER ***************************************************
#[derive(Debug)]
pub struct CustomHandler {
    num_attempts: usize,
}

impl CustomHandler {
    pub fn new(attempts: usize) -> Self {
        let max_attempts = cmp::min(MAX_ATTEMPTS, attempts);
        Self {
            num_attempts: max_attempts,
        }
    }
}

impl ErrorHandler<JonError> for CustomHandler {
    type OutError = JonError;

    fn handle(&mut self, attempt: usize, e: JonError) -> RetryPolicy<JonError> {
        if attempt == self.num_attempts {
            eprintln!("No attempts left");
            return RetryPolicy::ForwardError(e);
        } else {
            RetryPolicy::WaitRetry(Duration::from_secs(1))
        }
        // match e.kind() {
        //     io::ErrorKind::ConnectionRefused => RetryPolicy::WaitRetry(Duration::from_secs(1)),
        //     io::ErrorKind::TimedOut => RetryPolicy::Repeat,
        //     _ => RetryPolicy::ForwardError(e),
        // }
    }
}
