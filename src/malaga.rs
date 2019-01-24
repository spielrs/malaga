use super::{Next, Mdw};
use malaga_http_utils::utils::Methods;
use futures::Future;
use std::io;

type ResponseMdw<Res> = Box<Future<Item = Mdw<Res>, Error = io::Error> + Send>;

pub struct Middleware<Req, Res> {
    pub url: String,
    pub method: Methods,
    pub handler: fn(req: Req, next: Next) -> ResponseMdw<Res>,
}

pub struct Malaga<T, S> {
    middlewares: Vec<Middleware<T, S>>,
}

impl <Req, Res>Malaga<Req, Res> {
    pub fn new() -> Malaga<Req, Res> {
        Malaga {
            middlewares: vec!(),
        }
    }

    pub fn get(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::GET,
            });
    }

    pub fn post(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }

    pub fn put(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }
    
    pub fn delete(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        ResponseMdw<Res>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }

    pub fn mdw(&mut self, url: &str, method: Methods, handler: fn(req: Req, next: Next) ->
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
    use super::{Malaga, Mdw, Next};

    struct Body {
        username: String,
        password: String,
    }

    struct Req {
        method: String,
        body: Body,
    }

    struct Res;

    fn get_response(type_res: &str, res: Res, next: Next) -> Mdw<Res> {
        match type_res {
            "next" => Mdw::NEXT(next()),
            "res" => Mdw::RESP(res),
        }
    }

    #[test]
    fn push_middleware() {
        let mdlw = Malaga {
            middlewares: vec!(),
        };

        let res = Res {};

        mdlw.get("/user", |req: Req, next| {
            println!("Access to user");

            Box::new(Ok(get_response("next", res, next)).and_then(|data| data))
        });
    }
}
