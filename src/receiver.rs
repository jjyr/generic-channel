#[cfg(feature = "crossbeam_channel")]
mod crossbeam;
mod standard;
use std::{error, fmt};

pub trait Receiver<T> {
    fn try_recv(&self) -> Result<T, TryRecvError>;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TryRecvError {
    /// receiving on an empty channel
    Empty,
    /// receiving on an closed channel
    Disconnected,
}

impl fmt::Display for TryRecvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TryRecvError::Empty => "receiving on an empty channel".fmt(f),
            TryRecvError::Disconnected => "receiving on a closed channel".fmt(f),
        }
    }
}

impl error::Error for TryRecvError {
    fn description(&self) -> &str {
        match *self {
            TryRecvError::Empty => "receiving on an empty channel",
            TryRecvError::Disconnected => "receiving on a closed channel",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
