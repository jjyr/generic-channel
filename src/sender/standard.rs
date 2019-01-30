use crate::{Sender as SenderTrait, TrySendError};
use std::sync::mpsc::{Sender, SyncSender, TrySendError as SyncTrySendError};

impl<T> SenderTrait<T> for Sender<T> {
    fn try_send(&self, t: T) -> Result<(), TrySendError<T>> {
        self.send(t)
            .map_err(|err| TrySendError::Disconnected(err.0))
    }
}

impl<T> SenderTrait<T> for SyncSender<T> {
    fn try_send(&self, t: T) -> Result<(), TrySendError<T>> {
        self.try_send(t).map_err(|err| match err {
            SyncTrySendError::Full(t) => TrySendError::Full(t),
            SyncTrySendError::Disconnected(t) => TrySendError::Disconnected(t),
        })
    }
}
