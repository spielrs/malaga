use super::Next;
use malaga_http_utils::utils::Methods;
use futures::Future;
use std::io;


pub struct Middleware<Req, Res> {
    pub url: String,
    pub method: Methods,
    pub handler: fn(req: Req, next: Next) -> Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>,
}

pub struct Malaga<T, S> {
    middlewares: Vec<Middleware<T, S>>,
}

impl <Req, Res>Malaga<Req, Res> {
    pub fn get(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::GET,
            });
    }

    pub fn post(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }

    pub fn put(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }
    
    pub fn delete(&mut self, url: &str, handler: fn(req: Req, next: Next) ->
        Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method: Methods::POST,
            });
    }

    pub fn mdw(&mut self, url: &str, method: Methods, handler: fn(req: Req, next: Next) ->
        Box<Future<Item = Result<Res, Next>, Error = io::Error> + Send>) {
            self.middlewares.push(Middleware {
                url: url.to_string(),
                handler,
                method,
            });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Malaga;

//     struct Body {
//         username: String,
//         password: String,
//     }

//     struct Req {
//         method: String,
//         body: Body,
//     }

//     #[test]
//     fn push_middleware() {
//         let mdlw = Malaga {
//             middlewares: vec!(),
//         };

//         mdlw.get("/user", |req: Req, next| {
            
//         });
//     }
// }
