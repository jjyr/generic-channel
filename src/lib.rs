mod sender;
//mod receiver;

pub use crate::sender::{Sender, TrySendError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
