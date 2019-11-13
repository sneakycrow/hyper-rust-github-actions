extern crate hyper;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};

fn main() {
  pretty_env_logger::init();
  let addr = ([127, 0, 0, 1], 3000).into();

  let new_service = || service_fn_ok(|_req| Response::new(Body::from("Hello World!")));

  let server = Server::bind(&addr)
    .serve(new_service)
    .map_err(|e| eprintln!("server error: {}", e));

  hyper::rt::run(server);
}
