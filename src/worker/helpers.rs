use malaga_http_utils::utils::Methods;
use futures::{Future};
use std::io;
use super::worker::Worker;

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
}

pub struct Malaga<T, S> {
    middlewares: Vec<Middleware<T, S>>,
}

impl <Req, Res>Worker<Req, Res> for Malaga<Req, Res> {
    fn new() -> Malaga<Req, Res> {
        Malaga {
            middlewares: vec!(),
        }
    }

    fn get(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::GET,
            });
    }

    fn post(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }

    fn put(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }
    
    fn delete(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }

    fn mdw(&mut self, url: &str, method: Methods, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::{Malaga, Mdw, Next, ResponseMdw, Worker};
    use futures::future;

    struct Req {}

    struct Res;

    fn get_response(type_res: &str, res: Res, next: Next) -> Mdw<Res> {
        match type_res {
            "next" => Mdw::NEXT(next()),
            _ => Mdw::RESP(res),
        }
    }

    #[test]
    fn push_middleware() {
        let mut mdlws = Malaga {
            middlewares: vec!(),
        };

        fn user_access(_req: Req, next: Next) -> ResponseMdw<Res> {
            println!("Access to user");

            let res = Res {};

            Box::new(future::result(Ok(get_response("next", res, next))))
        }

        mdlws.get("/user", user_access);

        fn next () {};

        for mdlw in mdlws.middlewares {
            let req = Req {};
            (mdlw.handler)(req, next);
        }
    }
}