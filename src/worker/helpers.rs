use futures::{Poll, Async, Future};
use lapin_futures as lapin;
use lapin::{Channel, Queue, Error, ConfirmationFuture};

pub struct ConsumerSettings {
    pub channel_id: u16,
    pub channel: Channel,
    pub path_root: String,
    pub getQueue: ConfirmationFuture<Queue>
}

pub struct ConsumerSettingsResolved {
        pub channel_id: u16,
        pub channel: Channel,
        pub path_root: String,
        pub queue: Queue
}

impl <'a>Future for ConsumerSettings {
    type Item = ConsumerSettingsResolved;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let value = match self.getQueue.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };

        Ok(Async::Ready(ConsumerSettingsResolved {
            channel_id: self.channel_id,
            channel: self.channel.clone(),
            path_root: self.path_root.clone(),
            queue: value
        }))
    }
}