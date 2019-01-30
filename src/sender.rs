#[cfg(feature = "crossbeam-channel")]
mod crossbeam;
#[cfg(feature = "futures")]
mod futures;
mod standard;

#[cfg(feature = "futures")]
pub use crate::sender::futures::SenderWrapper as FuturesSender;
use std::{error, fmt};

pub trait Sender<T> {
    fn try_send(&self, t: T) -> Result<(), TrySendError<T>>;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TrySendError<T> {
    /// channel is full
    Full(T),
    /// send to a closed channel
    Disconnected(T),
}

impl<T> fmt::Debug for TrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrySendError::Full(..) => "Full(..)".fmt(f),
            TrySendError::Disconnected(..) => "Disconnected(..)".fmt(f),
        }
    }
}

impl<T> fmt::Display for TrySendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrySendError::Full(..) => "sending on a full channel".fmt(f),
            TrySendError::Disconnected(..) => "sending on a closed channel".fmt(f),
        }
    }
}

impl<T: Send> error::Error for TrySendError<T> {
    fn description(&self) -> &str {
        match *self {
            TrySendError::Full(..) => "sending on a full channel",
            TrySendError::Disconnected(..) => "sending on a closed channel",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
