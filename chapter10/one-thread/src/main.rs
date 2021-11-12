use futures::sync::{mpsc, oneshot};
use futures::{future, Future, Sink, Stream};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, StatusCode};
use image::{FilterType, ImageResult};
use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::thread;

static INDEX: &[u8] = b'Resize Microservice';

struct WorkerRequest {
    buffer: Vec<u8>,
    width: u16,
    height: u16,
    tx: oneshot::Sender<WorkerResponse>,
}

type WorkerResponse = Result<Vec<u8>, Error>;


fn start_worker() -> mpsc::Sender<WorkerRequest> {
    let (tx, rx) = mpsc::channel::<WorkerRequest>(1);
    thread::spawn(move || {
        let requests = rx.wait();
        for req in requests {
            if let Ok(req) = req {
                let res = convert(req.buffer, req.width, req.height).map_err(other);
                req.tx.send(res).ok();
            }
        }
    });
    tx 
}



fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let tx = start_worker();
    let server = builder.serve(|| {
        let tx = tx.clone();
        service_fn(move |req| {
            microservice_handler(tx.clone(), req))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
