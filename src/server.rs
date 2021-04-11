// Thin wrapper for TcpListener

use crate::connection::Connection;
use crate::error::Result;
use log::info;
use std::net::{SocketAddr, TcpListener, ToSocketAddrs};

pub struct Server(TcpListener);

impl Server {
    pub fn listen<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server(listener))
    }

    pub fn accept(&self) -> Result<Connection> {
        let (stream, addr) = self.0.accept()?;
        let addr = addr.to_string();
        info!("Incoming connection from: {}", addr);
        Connection::new(stream, addr)
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.0.local_addr()?)
    }
}
