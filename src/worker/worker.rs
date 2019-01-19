use malaga_http_utils::{utils::Methods};
use futures::Future;
use std::io;

pub type Next = fn();

/// Worker trait to implement all the middlewares
pub trait Worker {
    fn connect(&self, address: str, handler: fn());

    fn get<Req, Res>(&self, url: str, handler: fn(req: Req, next: Next) 
        -> Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>);

    fn post<Req, Res>(&self, url: str, handler: fn(req: Req, next: Next) 
        -> Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>);

    fn put<Req, Res>(&self, url: str, handler: fn(req: Req, next: Next) 
        -> Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>);

    fn delete<Req, Res>(&self, url: str, handler: fn(req: Req, next: Next) 
        -> Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>);

    fn mdw<Req, Res>(&self, url: str, method: Methods, handler: fn(req: Req, next: Next) 
        -> Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>);
}
