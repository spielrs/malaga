use futures::future::{AndThen, Future};
use lapin_futures as lapin;
use lapin::{ClientFuture, Channel, Queue, Client, Error};

pub trait Worker{
	fn new(&mut self);
}
