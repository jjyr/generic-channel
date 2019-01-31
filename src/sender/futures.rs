use crate::{Sender as SenderTrait, TrySendError};
use futures::sync::mpsc::Sender;

impl<T> SenderTrait<T> for Sender<T> {
    fn try_send(&mut self, t: T) -> Result<(), TrySendError<T>> {
        Sender::try_send(self, t).map_err(|err| {
            if err.is_full() {
                TrySendError::Full(err.into_inner())
            } else {
                TrySendError::Disconnected(err.into_inner())
            }
        })
    }
}
