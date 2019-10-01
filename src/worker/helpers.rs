use futures::{Poll, Async, Future};
use lapin_futures as lapin;
use lapin::{Channel, Queue, Error, ConfirmationFuture, Consumer};

pub struct ConsumerSettings {
    pub channel_id: u16,
    pub channel: Channel,
    pub path_root: String,
    pub get_queue: ConfirmationFuture<Queue>
}

pub struct ConsumerSettingsResolved {
    pub channel_id: u16,
    pub channel: Channel,
    pub path_root: String,
    pub queue: Queue
}

pub struct StreamSettings {
    pub channel: Channel,
    pub get_stream: Box<Future<Item = Consumer, Error = Error>>
}

pub struct StreamSettingsResolved {
    pub channel: Channel,
    pub stream: Consumer
}

impl Future for ConsumerSettings {
    type Item = ConsumerSettingsResolved;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let value = match self.get_queue.poll() {
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

impl Future for StreamSettings {
    type Item = StreamSettingsResolved;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let value = match self.get_stream.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };

        Ok(Async::Ready(StreamSettingsResolved {
            channel: self.channel.clone(),
            stream: value,
        }))
    }
}