extern crate failure;
extern crate futures;
extern crate tokio;

use failure::Error;
use futures::sync::{mpsc, oneshot};
use futures::{future, stream, Future, IntoFuture, Sink, Stream};
use std::io;
use tokio::codec::LinesCodec;
use tokio::net::{UdpFramed, UdpSocket};

fn main() {
    single();
    multiple();
    send_spawn();
    println!("Start UDP echo");
    alt_udp_echo().unwrap();
}

fn to_box<T>(fut: T) -> Box<dyn Future<Item = (), Error = ()> + Send>
where
    T: IntoFuture,
    T::Future: Send + 'static,
    T::Item: 'static,
    T::Error: 'static,
{
    let fut = fut.into_future().map(drop).map_err(drop);
    Box::new(fut)
}

fn single() {
    let (tx_sender, rx_future) = oneshot::channel::<u8>();
    let receiver = rx_future.map(|x| println!("Received: {}", x));
    let sender = tx_sender.send(8);
    let execute_all = future::join_all(vec![to_box(receiver), to_box(sender)]).map(drop);
    tokio::run(execute_all);
}