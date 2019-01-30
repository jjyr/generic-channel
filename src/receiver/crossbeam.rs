use crate::{Receiver as ReceiverTrait, TryRecvError};
use crossbeam_channel::{Receiver, TryRecvError as CBTryRecvError};

impl<T> ReceiverTrait<T> for Receiver<T> {
    fn try_recv(&self) -> Result<T, TryRecvError> {
        self.try_recv().map_err(|err| match err {
            StdTryRecvError::Empty => TryRecvError::Empty,
            StdTryRecvError::Disconnected => TryRecvError::Disconnected,
        })
    }
}
