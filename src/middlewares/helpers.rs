use malaga_http_utils::utils::Methods;
use futures::{Future};
use std::io;
use super::mdw::Middlewares;

pub type Next = fn();

pub enum Mdw<Res> {
    NEXT(()),
    RESP(Res),
}

pub type ResponseMdw<Res> = Box<Future<Item = Mdw<Res>, Error = io::Error> + Send>;

pub struct Middleware<Req, Res> {
    pub url: String,
    pub method: Methods,
    pub handler: fn(req: Req, next: Next) -> ResponseMdw<Res>,
    pub request: Req,
}

pub struct MalagaMdw<T, S> {
    middlewares: Vec<Middleware<T, S>>,
}

impl <Req, Res>Middlewares<Req, Res> for MalagaMdw<Req, Res> {
    fn new() -> MalagaMdw<Req, Res> {
        MalagaMdw {
            middlewares: vec!(),
        }
    }

    fn get(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::GET,
                request: req,
            });
    }

    fn post(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
                request: req,
            });
    }

    fn put(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
                request: req,
            });
    }
    
    fn delete(&mut self, url: &str, req: Req, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
                request: req,
            });
    }

    fn mdw(&mut self, url: &str, req: Req, method: Methods, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method,
                request: req,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::{MalagaMdw, Mdw, Next, ResponseMdw, Middlewares};
    use futures::future;

    struct Req {}

    struct Res;

    #[test]
    fn push_middleware() {
        let mut mdlws = MalagaMdw {
            middlewares: vec!(),
        };

        fn user_access(_req: Req, next: Next) -> ResponseMdw<Res> {
            println!("Access to user");

            Box::new(future::result(Ok(Mdw::NEXT(next()))))
        }

        let req = Req {};

        mdlws.get("/user", req, user_access);

        fn next () {};

        for mdlw in mdlws.middlewares {
            (mdlw.handler)(mdlw.request, next);
        }
    }
}