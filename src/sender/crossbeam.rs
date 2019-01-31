use crate::{Sender as SenderTrait, TrySendError};
use crossbeam_channel::{Sender, TrySendError as CBTrySendError};

impl<T> SenderTrait<T> for Sender<T> {
    fn try_send(&mut self, t: T) -> Result<(), TrySendError<T>> {
        Sender::try_send(self, t).map_err(|err| match err {
            CBTrySendError::Full(t) => TrySendError::Full(t),
            CBTrySendError::Disconnected(t) => TrySendError::Disconnected(t),
        })
    }
}
