mod receiver;
mod sender;

pub use crate::receiver::{Receiver, TryRecvError};
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
