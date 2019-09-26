use futures::future::{AndThen, Future};
use lapin_futures as lapin;
use lapin::{ClientFuture, Channel, Error};

pub trait Worker{
	fn new(&mut self);
	fn create_connection(&mut self) -> AndThen<ClientFuture, Box<Future<Item = Channel, Error = Error> + Send>, Channel>;
	fn get_request_from_provider(&mut self);
	fn execute_middlewares(&mut self);
	fn response_to_provider(&mut self);
}
