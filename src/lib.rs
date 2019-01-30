//! Generic-channel provides common abstract traits [`Sender`], [`Receiver`] between several channel implementations that widely used in the Rust community.
//!
//! Currently support channel implementations:
//!
//! * std
//! * crossbeam-channel
//! * futures
//!
//! NOTE: you need to enable those features in `Cargo.toml`, use `all` flag to enable all.
//!
//! A handler function only wants to handle a sender or receiver to send/recv messages and do not
//! care about the actual type or whether the sender is a crossbeam sender or a futures sender.
//!
//! # Examples
//!
//! ```rust
//! # extern crate generic_channel;
//! # extern crate crossbeam_channel;
//! use generic_channel::{Sender, TrySendError};
//!
//! // this method do not care about sender type.
//! fn event_producer<S: Sender<i32>>(sender: S) -> Result<(), TrySendError<i32>> {
//!     for i in 1..10 {
//!         sender.try_send(i)?
//!     }
//!     Ok(())
//! }
//!
//! // we can pass crossbeam channel to event_producer
//! let (sender, receiver) = crossbeam_channel::unbounded::<i32>();
//! event_producer(sender);
//! assert_eq!((1..10).map(|_| receiver.recv().unwrap()).collect::<Vec<_>>(), (1..10).collect::<Vec<_>>());
//!
//! // we can also pass a std Sender or a futures Sender
//! let (sender, receiver) = std::sync::mpsc::channel::<i32>();
//! event_producer(sender);
//! assert_eq!((1..10).map(|_| receiver.recv().unwrap()).collect::<Vec<_>>(), (1..10).collect::<Vec<_>>());
//! ```
//!
mod receiver;
mod sender;

pub use crate::receiver::{Receiver, TryRecvError};
#[cfg(feature = "futures")]
pub use crate::sender::FuturesSender;
pub use crate::sender::{Sender, TrySendError};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    #[test]
    fn sender_works() {
        let (sender, receiver) = channel();
        let sender: Box<dyn Sender<u8>> = Box::new(sender);
        sender.try_send(1).expect("send");
        assert_eq!(receiver.recv().expect("recv"), 1u8);
        drop(receiver);
        assert_eq!(sender.try_send(2), Err(TrySendError::Disconnected(2u8)));
    }

    #[test]
    fn receiver_works() {
        let (sender, receiver) = channel();
        let receiver: Box<dyn Receiver<u8>> = Box::new(receiver);
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
        sender.send(1).expect("send");
        assert_eq!(receiver.try_recv().expect("recv"), 1u8);
        drop(sender);
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));
    }
}
