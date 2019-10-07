use futures::{Future};
use lapin_futures as lapin;
use lapin::{BasicProperties, Client, ConnectionProperties};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use log::{info};
use super::provider::Provider;

pub struct MalagaPvr;

impl Provider for MalagaPvr {
    fn new (&mut self, addr: &str, path_root: &str, request: &str) {
        futures::executor::spawn(
            Client::connect(&addr, ConnectionProperties::default())
            .and_then(|client| {
                client.create_channel()
            })
            .and_then(|channel| {
                let id = channel.id();
                info!("created channel with id: {}", id);

                channel.queue_declare(path_root, QueueDeclareOptions::default(), FieldTable::default())
                    .and_then(move |_| {
                        channel.basic_publish("", path_root, request.as_bytes().to_vec(), BasicPublishOptions::default(), BasicProperties::default())
                    })
            })
        )
        .wait_future().expect("runtime failure");
    }
}