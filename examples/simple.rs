// You can run this example by doing `cargo run --example simple`
// Hint: Tendermint 0.34.x must running

use std::net::TcpListener;
use std::net::TcpStream;
use abci2_rs::codec::ServerCodec;

use simple_logger::SimpleLogger;
use log::{error, info, trace};
use tendermint_proto::abci::{
    RequestApplySnapshotChunk, RequestBeginBlock, RequestCheckTx, RequestDeliverTx,
    RequestEcho, RequestEndBlock, RequestInfo, RequestLoadSnapshotChunk,
    RequestOfferSnapshot, RequestQuery, RequestSetOption, response, Response,
    ResponseApplySnapshotChunk, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEcho, ResponseEndBlock, ResponseFlush, ResponseInfo,
    ResponseInitChain, ResponseListSnapshots, ResponseLoadSnapshotChunk, ResponseOfferSnapshot,
    ResponseQuery, ResponseSetOption,
};
use tendermint_proto::abci::request::Value;
use std::thread;

fn main() {
    SimpleLogger::new().init().unwrap();

    let listener = TcpListener::bind("127.0.0.1:26658").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.local_addr().unwrap().clone();

        thread::spawn(move || {
            handle_connection(stream, addr.to_string());
        });
    }
}

fn handle_connection(stream: TcpStream, addr: String) {
    let mut codec = ServerCodec::new(stream, 1024*1024);

    info!("Listening for incoming requests from {}", addr);
    loop {
        let req = match codec.next() {
            Some(result) => match result {
                Ok(r) => r,
                Err(e) => {
                    error!(
                        "Failed to read incoming request from client {}: {:?}",
                        addr, e
                    );
                    return;
                }
            },
            None => {
                info!("Client {} terminated stream", addr);
                return;
            }
        };

        let req_clone = req.clone();
        
        let res = Response {
            value: Some(match req.value.unwrap() {
                Value::Echo(req) => response::Value::Echo(echo(req)),
                Value::Flush(_) => response::Value::Flush(flush()),
                Value::Info(req) => response::Value::Info(info(req)),
                Value::SetOption(req) => response::Value::SetOption(set_option(req)),
                Value::InitChain(req) => {
                    response::Value::InitChain { 0: ResponseInitChain {
                        consensus_params: None,
                        validators: vec![],
                        app_hash: vec![]
                    } }
                },
                Value::Query(req) => response::Value::Query(query(req)),
                Value::BeginBlock(req) => response::Value::BeginBlock(begin_block(req)),
                Value::CheckTx(req) => response::Value::CheckTx(check_tx(req)),
                Value::DeliverTx(req) => response::Value::DeliverTx(deliver_tx(req)),
                Value::EndBlock(req) => response::Value::EndBlock(end_block(req)),
                Value::Commit(_) => response::Value::Commit(commit()),
                Value::ListSnapshots(_) => response::Value::ListSnapshots(list_snapshots()),
                Value::OfferSnapshot(req) => {
                    response::Value::OfferSnapshot(offer_snapshot(req))
                }
                Value::LoadSnapshotChunk(req) => {
                    response::Value::LoadSnapshotChunk(load_snapshot_chunk(req))
                }
                Value::ApplySnapshotChunk(req) => {
                    response::Value::ApplySnapshotChunk(apply_snapshot_chunk(req))
                }

            })
        };

        let res_clone = res.clone();

        if let Err(e) = codec.send(res_clone) {
            error!("Failed sending response to client {}: {:?}", addr, e);
            return;
        }


        trace!("<< {:?}", req_clone);
        trace!("<< {:?}", res);
    }

    fn echo(request: RequestEcho) -> ResponseEcho {
        ResponseEcho {
            message: request.message,
        }
    }
    /// Provide information about the ABCI application.
    fn info(_request: RequestInfo) -> ResponseInfo {
        Default::default()
    }

    /// Query the application for data at the current or past height.
    fn query(_request: RequestQuery) -> ResponseQuery {
        Default::default()
    }

    /// Check the given transaction before putting it into the local mempool.
    fn check_tx(_request: RequestCheckTx) -> ResponseCheckTx {
        Default::default()
    }

    /// Signals the beginning of a new block, prior to any `DeliverTx` calls.
    fn begin_block(_request: RequestBeginBlock) -> ResponseBeginBlock {
        Default::default()
    }

    /// Apply a transaction to the application's state.
    fn deliver_tx(_request: RequestDeliverTx) -> ResponseDeliverTx {
        Default::default()
    }

    /// Signals the end of a block.
    fn end_block(_request: RequestEndBlock) -> ResponseEndBlock {
        Default::default()
    }

    /// Signals that messages queued on the client should be flushed to the server.
    fn flush() -> ResponseFlush {
        ResponseFlush {}
    }

    /// Commit the current state at the current height.
    fn commit() -> ResponseCommit {
        Default::default()
    }

    /// Allows the Tendermint node to request that the application set an
    /// option to a particular value.
    fn set_option( _request: RequestSetOption) -> ResponseSetOption {
        Default::default()
    }

    /// Used during state sync to discover available snapshots on peers.
    fn list_snapshots() -> ResponseListSnapshots {
        Default::default()
    }

    /// Called when bootstrapping the node using state sync.
    fn offer_snapshot(_request: RequestOfferSnapshot) -> ResponseOfferSnapshot {
        Default::default()
    }

    /// Used during state sync to retrieve chunks of snapshots from peers.
    fn load_snapshot_chunk(_request: RequestLoadSnapshotChunk) -> ResponseLoadSnapshotChunk {
        Default::default()
    }

    /// Apply the given snapshot chunk to the application's state.
    fn apply_snapshot_chunk(
        _request: RequestApplySnapshotChunk,
    ) -> ResponseApplySnapshotChunk {
        Default::default()
    }

}