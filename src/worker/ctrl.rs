use super::worker::Worker;
use super::super::middlewares::ctrl::MalagaMdw;
use std::io;
use futures::future::{Future, AndThen};
use lapin_futures as lapin;
use lapin::{BasicProperties, Channel, ClientFuture, Client, ConnectionProperties, Error};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use log::info;

pub struct MalagaWk<Req, Res> {
    path_root: String,
    connection_url: String,
    middlewares: MalagaMdw<Req, Res>
}

impl <Req, Res>Worker for MalagaWk<Req, Res>{
    fn new(&mut self) {
        futures::executor::spawn(
            Self::create_connection(self)
        );
    }

    fn create_connection(&mut self) -> AndThen<ClientFuture, Box<Future<Item = Channel, Error = Error> + Send>, Channel> {
        Client::connect(format!("{}/{}", self.connection_url, self.path_root), ConnectionProperties::default())
            .and_then(|client| {
                client.create_channel()
            })
    }
}
