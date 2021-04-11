// There are exists several versions of ABCI Wrappers
// This version is reduced to cover the requirements for the Nomic Bitcoin PegZone

use std::net::TcpStream;
use tendermint_proto::abci::{Request, Response};

use crate::codec::ServerCodec;
use crate::error::Result;
use log::{debug, error, info};

pub const MAX_MESSAGE_LENGTH: usize = 1024 * 1024; // TODO: make configurable?

pub struct Connection {
    stream: ServerCodec<TcpStream>,
    local_addr: String,
}

impl Connection {
    pub fn new(stream: TcpStream, addr: String) -> Result<Self> {
        let codec = ServerCodec::new(stream, 1024 * 1024);

        Ok(Connection {
            stream: codec,
            local_addr: addr,
        })
    }

    /// Getter for this server's local address.
    pub fn local_addr(&self) -> String {
        self.local_addr.clone()
    }

    pub fn read(&mut self) -> Result<Request> {
        debug!("Read: Listening for incoming requests from client");
        let request = match self.stream.next() {
            Some(result) => match result {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to read incoming request from client: {:?}", e);
                    return Ok(Default::default());
                }
            },
            None => {
                info!("Client terminated stream");
                return Ok(Default::default());
            }
        };

        Ok(request)
    }

    pub fn write(&mut self, res: Response) -> Result<()> {
        debug!("Write entered!");
        if let Err(e) = self.stream.send(res) {
            error!("Failed sending response to client {:?}", e);
            return Ok(Default::default());
        }
        Ok(())
    }

    pub fn close(mut self) -> Result<()> {
        self.end()
    }

    fn end(&mut self) -> Result<()> {
        //self.stream.shutdown(std::net::Shutdown::Both);
        // read and write threads will end as the connection will now error when
        // trying to use the socket or channels, whichever happens first
        Ok(())
    }

    pub fn stream(&self) -> &ServerCodec<TcpStream> {
        &self.stream
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        match self.end() {
            // swallow NotConnected errors since we want to disconnect anyway
            // TODO:
            //Err(err) if err.as_fail() == std::io::ErrorKind::NotConnected
            //     => {},
            Err(_err) => {}
            _ => {}
        };
    }
}
