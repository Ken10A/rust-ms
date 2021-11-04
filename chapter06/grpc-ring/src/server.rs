mod ring;
mod ring_grpc;

use crate::ring::Empty;
use crate::ring_grpc::{Ring, RingServer};
use failure::Error;
use grpc::{Error as GrpcError, RequestOptions, ServerBuilder, SingleResponse};
use grpc_ring::Remote;
use log::{debug, trace};
use std::env;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

macro_rules! try_or_response {
    ($x:expr) => {{
        match $x {
            Ok(value) => value,
            Err(err) => {
                let error = GrpcError::Panic(err.to_string());
                return SingleResponse::err(error);
            }
        }
    }};
}

enum Action {
    StartRollCall,
    MarkItself,
}

struct RingImpl {
    sender: Mutex<Sender<Action>>,
}

impl RingImpl {
    fn new(sender: Sender<Action>) -> Self {
        Self {
            sender: Mutex::new(sender),
        }
    }

    fn send_action(&self, action: Action) -> SingleResponse<Empty> {
        let tx = try_or_response!(self.sender.lock());
        try_or_response!(tx.send(action));
        let result = Empty::new();
        SingleResponse::completed(result)
    }
}
