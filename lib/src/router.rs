use std::net::SocketAddr;

use futures::future;
use hyper::{self, rt::Future, service::service_fn, Body, Request, Response, Server, StatusCode};

use super::route::Route;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
    // pub fn get<S: Into<String>>(&mut self, pattern: S) {
    //     self.add(Method::GET, pattern)
    // }
    // pub fn add<S: Into<String>>(&mut self, method: Method, pattern: S) {
    //     self.routes.push(Route::new(method, pattern.into()))
    // }
    pub fn walk() {}
    // fn handler<F>(&self) -> (Fn(Request<Body>) -> BoxFut) {
    //     let routes = Arc::clone(&self.routes);
    //     let routes = routes.lock().unwrap();
    //     |req: Request<Body>| -> BoxFut {
    //         let mut response = Response::new(Body::empty());
    //         let mut ok = false;
    //         for rt in routes.iter() {
    //             if rt.is_match(req.method(), req.uri().path()) {
    //                 *response.body_mut() = Body::from("matched");
    //                 ok = true;
    //             }
    //         }
    //         if !ok {
    //             *response.status_mut() = StatusCode::NOT_FOUND;
    //         }
    //
    //         Box::new(future::ok(response))
    //     }
    // }

    // pub fn run(self, addr: &SocketAddr) {
    //     let server = Server::bind(addr).serve(move || self.handler());
    //     info!("Listening on http://{}", addr);
    //     hyper::rt::run(server.map_err(|e| error!("server error: {}", e)));
    // }
}

//-----------------------------------------------------------------------------
pub fn run(addr: &'static SocketAddr, router: &'static Router) {
    let service = move || {
        service_fn(move |req: Request<Body>| -> BoxFut {
            let mut response = Response::new(Body::empty());
            let mut ok = false;

            for rt in router.routes.iter() {
                if rt.is_match(req.method(), req.uri().path()) {
                    *response.body_mut() = Body::from("matched");
                    ok = true;
                }
            }

            if !ok {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }

            Box::new(future::ok(response))
        })
    };
    hyper::rt::run(hyper::rt::lazy(move || {
        let server = Server::try_bind(addr)
            .unwrap()
            .serve(service)
            .map_err(|e| error!("server error: {}", e));
        info!("Listening on http://{}", addr);
        hyper::rt::spawn(server);

        Ok(())
    }));
    // hyper::rt::run(server.map_err(|e| error!("server error: {}", e)));
}
