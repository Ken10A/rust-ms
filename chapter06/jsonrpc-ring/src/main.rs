use failure::Error;
use jsonrpc::client::Client;
use jsonrpc::error::Error as ClientError;
use jsonrpc_http_server::jsonrpc_core::{Error as ServerError, IoHandler, Value};
use jsonrpc_http_server::ServerBuilder;
use log::{debug, error, trace};
use serde::Deserialize;
use std::env;
use std::fmt;
use std::net::SocketAddr;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread;

const START_ROLL_CALL: &str = "start_roll_call";
const MARK_ITSELTF: &str = "mark_itself";

enum Action {
    StartRollCall,
    MarkItself,
}

struct Remote {
    client: Client,
}

impl Remote {
    fn new(addr: SocketAddr) -> Self {
        let url = format!("http://{}", addr);
        let client = Client::new(url, None, None);
        Self { client }
    }

    fn start_roll_call(&self) -> Result<bool, ClientError> {
        self.call_method(START_ROLL_CALL, &[])
    }
    fn mark_itself(&self) -> Result<bool, ClientError> {
        self.call_method(MARK_ITSELTF, &[])
    }
    fn call_method<T>(&self, meth: &str, args: &[Value]) -> Result<T, ClientError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = self.client.build_request(meth, args);
        self.client
            .send_request(&request)
            .and_then(|res| res.into_result::<T>())
    }
}

fn to_internal<E: fmt::Display>(err: E) -> ServerError {
    error!("Error: {}", err);
    ServerError::internal_error()
}

fn main() {
    println!("Hello, world!");
}
