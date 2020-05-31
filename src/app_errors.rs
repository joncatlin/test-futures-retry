use futures_retry::{ErrorHandler, RetryPolicy};
use std::io;
use std::time::Duration;
use std::cmp;

//use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, AppError>;

//const MAX_ATTEMPTS: usize = 5;
static BACKOFF_DURATIONS: &'static [u64] = &[50, 250, 1000, 10000, 30000];

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    General,
    FooBar,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        debug!("In impl From<io::Error> for AppError. error={:?}", error);
        AppError::IoError(error)
    }
}

// impl From<()> for AppError {
//     fn from(error: ()) -> Self {
//         debug!("In impl From<()> for AppError. error={:?}", error);
//         AppError::General
//     }
// }

impl From<(AppError, usize)> for AppError {
    fn from((error, attempts): (AppError, usize)) -> Self {
        debug!("In impl From<(AppError, usize)> for AppError. error={:?} attempts={}", error, attempts);
        AppError::General
    }
}

impl fmt::Display for AppError {
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
    pub fn new(attempts: Option<usize>) -> Self {
        
        let max_attempts = match attempts {
            Some(n) => cmp::min(BACKOFF_DURATIONS.len(), n),
            None => BACKOFF_DURATIONS.len(),
        };
        Self {
            num_attempts: max_attempts,
        }
    }
}

impl ErrorHandler<AppError> for CustomHandler {
    type OutError = AppError;

    fn handle(&mut self, attempt: usize, e: AppError) -> RetryPolicy<AppError> {
        if attempt == self.num_attempts {
            debug!("ErrorHandler - No attempts left, returning error. {} attemp(s) have already been made.", self.num_attempts);
            return RetryPolicy::ForwardError(e);
        } else {
            match e {
                AppError::General => RetryPolicy::WaitRetry(Duration::from_millis(BACKOFF_DURATIONS[attempt-1])),
                AppError::IoError(e) => RetryPolicy::Repeat,
                _ => RetryPolicy::ForwardError(e),
            }
        }
    }
}