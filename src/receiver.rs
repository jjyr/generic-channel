pub trait Receiver<T> {
    type Error;
    fn try_recv(&self, t: T) -> Result<(), Self::Error>;
}

