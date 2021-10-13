extern crate futures;
extern crate hyper;
extern crate rand;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::ops::Range;
use futures::{future, Future, Stream};
use hyper::{Body, Error, Method, Response, Server, StatusCode};
use hyper::service::service_fn;
use rand::Rng;
use rand::distributions::{Bernoulli, Normal, Uniform};

static INDEX: &[u8] = b"Random Microservice";

#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]


fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| {
        service_fn(microservice_handler)
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}