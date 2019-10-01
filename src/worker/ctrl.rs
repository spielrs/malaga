use super::worker::{Worker};
use super::helpers::{ConsumerSettings, ConsumerSettingsResolved, StreamSettingsResolved, StreamSettings};
use super::super::middlewares::ctrl::MalagaMdw;
use std::str;
use futures::{Future, Stream};
use lapin_futures as lapin;
use lapin::{Channel, Client, ConnectionProperties, Error};
use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use log::{debug, info};

pub struct MalagaWk<Req, Res> {
    path_root: String,
    connection_url: String,
    middlewares: MalagaMdw<Req, Res>
}

fn create_channel(client: Client) -> Box<Future<Item = Channel, Error = Error>> {
    Box::new(client.create_channel())
}

fn create_stream(consumer_settings: ConsumerSettingsResolved) -> Box<Future<Item = StreamSettingsResolved, Error = Error>> {
    let channel = consumer_settings.channel;
    let queue = consumer_settings.queue;
    info!("create channel with id: {}", consumer_settings.channel_id);

    Box::new(StreamSettings {
        channel: channel.clone(),
        get_stream: Box::new(channel.basic_consume(&queue, &consumer_settings.path_root, BasicConsumeOptions::default(), FieldTable::default()))
    })
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
                            get_queue: channel.queue_declare(&self.path_root, QueueDeclareOptions::default(), FieldTable::default())
                        }
                )
            }))
            .and_then(Box::new(create_stream))
            .and_then(move |stream_settings| {
                stream_settings.stream.clone().for_each(move |message| {
                    debug!("got message: {:?}", message);
                    info!("decoded message: {:?}", str::from_utf8(&message.data).unwrap());
                    stream_settings.channel.basic_ack(message.delivery_tag, false)
                })
            })
        );
    }
}
