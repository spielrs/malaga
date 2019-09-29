use super::worker::{Worker};
use super::helpers::{ConsumerSettings, ConsumerSettingsResolved};
use super::super::middlewares::ctrl::MalagaMdw;
use std::io;
use futures::future::{Future};
use lapin_futures as lapin;
use lapin::{BasicProperties, Channel, Consumer, Queue, Client, ConnectionProperties, ConfirmationFuture, Error};
use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;

pub struct MalagaWk<Req, Res> {
    path_root: String,
    connection_url: String,
    middlewares: MalagaMdw<Req, Res>
}

fn create_stream(consumer_settings: ConsumerSettingsResolved) -> Box<Future<Item = Consumer, Error = Error>> {
    let channel = consumer_settings.channel;
    let queue = consumer_settings.queue;

    Box::new(channel.basic_consume(&queue, &consumer_settings.path_root, BasicConsumeOptions::default(), FieldTable::default()))
}

fn create_channel(client: Client) -> Box<Future<Item = Channel, Error = Error>> {
    Box::new(client.create_channel())
}

impl <Req, Res>Worker for MalagaWk<Req, Res>{
    fn new(&mut self) {
        futures::executor::spawn(
            Client::connect(&format!("{}/{}", self.connection_url, self.path_root), ConnectionProperties::default())
            .and_then(Box::new(create_channel))
            .and_then(Box::new(move |channel: Channel| {
                    Box::new(
                        ConsumerSettings {
                            channel_id: channel.id(),
                            channel: channel.clone(),
                            path_root: self.path_root.clone(),
                            getQueue: channel.queue_declare(&self.path_root, QueueDeclareOptions::default(), FieldTable::default())
                        }
                )
            }))
            .and_then(Box::new(create_stream))
        );
    }
}
