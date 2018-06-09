extern crate env_logger;
extern crate h2o;
#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;

lazy_static! {
    static ref ADDRESS: SocketAddr = ([127, 0, 0, 1], 8080).into();
    static ref ROUTER: h2o::router::Router = h2o::router::Router::new();
}

fn main() {
    env_logger::init();
    h2o::app::run(&ADDRESS, &ROUTER);
}
