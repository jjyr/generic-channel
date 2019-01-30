# Generic-channel
[![Crates.io](https://img.shields.io/crates/v/generic-channel.svg)](https://crates.io/crates/generic-channel)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://travis-ci.org/jjyr/generic-channel.svg?branch=master)](https://travis-ci.org/jjyr/generic-channel)
[Docs](https://docs.rs/generic-channel)

Generic-channel provides common abstract traits [`Sender`], [`Receiver`] between several channel implementations that widely used in the Rust community.

Currently support channel implementations:

* std
* crossbeam-channel
* futures

NOTE: you need to enable those features in `Cargo.toml`, use `all` flag to enable all.

A handler function only wants to handle a sender or receiver to send/recv messages and do not
care about the actual type or whether the sender is a crossbeam sender or a futures sender.

# Examples

```rust
# extern crate generic_channel;
# extern crate crossbeam_channel;
use generic_channel::{Sender, TrySendError};

// this method do not care about sender type.
fn event_producer<S: Sender<i32>>(sender: S) -> Result<(), TrySendError<i32>> {
    for i in 1..10 {
        sender.try_send(i)?
    }
    Ok(())
}

// we can pass crossbeam channel to event_producer
let (sender, receiver) = crossbeam_channel::unbounded::<i32>();
event_producer(sender);
assert_eq!((1..10).map(|_| receiver.recv().unwrap()).collect::<Vec<_>>(), (1..10).collect::<Vec<_>>());

// we can also pass a std Sender or a futures Sender
let (sender, receiver) = std::sync::mpsc::channel::<i32>();
event_producer(sender);
assert_eq!((1..10).map(|_| receiver.recv().unwrap()).collect::<Vec<_>>(), (1..10).collect::<Vec<_>>());
```

# Documentation

[Generic-channel Documentation](https://docs.rs/generic-channel)

# License

MIT

