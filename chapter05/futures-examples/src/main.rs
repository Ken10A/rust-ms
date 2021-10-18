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

fn send_spawn() {
    let (tx_sink, rx_stream) = mpsc::channel::<u8>(8);
    let receiver = rx_stream.fold(0, |acc, value| {
        println!("Received: {}", value);
        future::ok(acc + value)
    }).map(drop);

    let spawner = stream::iter_ok::<_, ()>(1u8..11u8).map(move |x| {
        let fut = tx_sink.clone().send(x).map(drop).map_err(drop);
        tokio::spawn(fut);
    }).collect();

    let execute_all = future::join_all(vec![
        to_box(spawner),
        to_box(receiver),
    ]).map(drop);
    tokio::run(execute_all);
}