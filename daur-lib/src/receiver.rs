use std::sync::mpsc::{SendError, Sender};

// TODO: change to ActionQueue
/// A type that can receive values.
pub trait Receiver<T> {
    /// Sends a value to the receiver.
    fn send(&mut self, value: T);
}

impl<T, R: Receiver<T>> Receiver<T> for &mut R {
    fn send(&mut self, value: T) {
        (*self).send(value);
    }
}

impl<T> Receiver<T> for Vec<T> {
    fn send(&mut self, value: T) {
        self.push(value);
    }
}

impl<T> Receiver<T> for Sender<T> {
    fn send(&mut self, value: T) {
        if let Err(SendError(value)) = Sender::send(self, value) {
            // The other end has been disconnected
            drop(value);
        }
    }
}
