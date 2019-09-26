extern crate malaga_http_utils;
extern crate futures;

pub mod middlewares;

pub use middlewares::ctrl::{Next, Mdw, MalagaMdw};
pub use middlewares::mdw::Middlewares;

pub mod worker;
pub use worker::ctrl::MalagaWk;
pub use worker::worker::Worker;
