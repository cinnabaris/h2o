use std::net::SocketAddr;

use futures::future;
use hyper::{self, rt::Future, service::service_fn, Body, Request, Response, Server};

use super::router::Router;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn run(addr: &'static SocketAddr, router: &'static Router) {
    let service = move || {
        service_fn(move |req: Request<Body>| -> BoxFut {
            let res = router.handle(req);
            Box::new(future::ok(res))
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
}
