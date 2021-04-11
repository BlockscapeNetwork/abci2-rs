# abci2-rs

*Simplified Low-level ABCI protocol server*


This crate provides low-level access to the ABCI protocol, via a `Connection` type which exposes `read()` and `write()` methods which return or accept ABCI request or response structs.
The hard work is done by [tendermint-rs](https://github.com/informalsystems/tendermint-rs) crate.

Currently supports **Tendermint 0.34.x**.

## Usage

**Add this crate as a dependency:**
```
[dependencies]
abci2-rs = "0.1"
```

**Example:**
```rust
// listen for ABCI connections from Tendermint
let server = abci2::Server::listen("localhost:26658").unwrap();

// wait for Tendermint to connect (note that we will need to accept the 3
// separate connections that Tendermint makes). this function blocks until
// a connection comes in.
let connection = server.accept().unwrap();

loop {
    // get an incoming request
    let req = connection.read().unwrap();

    // handle the request somehow
    let res = process_request();

    // send back the response
    connection.write(res).unwrap();
}
```

For a more complete example, see [examples/simple.rs](https://github.com/nomic-io/abci2/blob/master/examples/simple.rs) (you can run it via `cargo run --example simple`).


** Alternatives **
This crate does not replace any of the excellent and well written existing ABCI wrappers for Tendermint.
If you need a full documented and feature rich alternative you may find here [tendermint-abci](https://github.com/informalsystems/tendermint-rs/tree/master/abci) or here [abci-rs](https://github.com/devashishdxt/abci-rs) are perfect solution.
This version is tailored down for a dedicated use case and all credits goes to [Mappum] (Matt Bell <mappum@gmail.com>).

