use malaga_http_utils::utils::Methods;
use super::helpers::{ResponseMdw, Next};

pub trait Worker<Req, Res> {
    fn new() -> Self;
    fn get(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) -> ResponseMdw<Res>);
    fn post(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) -> ResponseMdw<Res>);
    fn put(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) -> ResponseMdw<Res>);
    fn delete(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) -> ResponseMdw<Res>);
    fn mdw(&mut self, url: &str, req: Req, method: Methods, handler: fn(req: Req, next: Next) -> ResponseMdw<Res>);
}