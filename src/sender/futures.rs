use crate::{Sender as SenderTrait, TrySendError};
use futures::sync::mpsc::Sender;
use std::sync::RwLock;

#[derive(Debug)]
pub struct SenderWrapper<T>(RwLock<Sender<T>>);

impl<T> From<Sender<T>> for SenderWrapper<T> {
    fn from(sender: Sender<T>) -> Self {
        SenderWrapper(RwLock::new(sender))
    }
}

impl<T> Clone for SenderWrapper<T> {
    fn clone(&self) -> Self {
        let sender = self.0.read().unwrap();
        SenderWrapper(RwLock::new(sender.clone()))
    }
}

impl<T> SenderTrait<T> for SenderWrapper<T> {
    fn try_send(&self, t: T) -> Result<(), TrySendError<T>> {
        let mut sender = self.0.write().unwrap();
        sender.try_send(t).map_err(|err| {
            if err.is_full() {
                TrySendError::Full(err.into_inner())
            } else {
                TrySendError::Disconnected(err.into_inner())
            }
        })
    }
}
