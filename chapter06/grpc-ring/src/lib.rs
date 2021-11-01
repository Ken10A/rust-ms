mod ring;
mod ring_grpc;

use crate::ring::Empty
use crate::ring_grpc::{Ring, RingClient};
use grpc::{ClientConf, ClientStubExt, Error as GrpcError, RequestOptions};
use std::net::SocketAddr;

pub struct Remote {
    client: RingClient
}

impl Remote {
    pub fu new(addr: SocketAddr) -> Result<Self, GrpcError> {
        let host = addr.ip().to_string();
        let port = addr.port();
        let conf = ClientConf::default();
        let client = RingClient::new_plain(&host, port, conf)?;
        Ok(Self {
            client
        })
    }
}