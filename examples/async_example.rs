extern crate tarantool;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate service_fn;

use tarantool::{Value, SyncClient, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation, Upsert,
                UpsertOperation};

use futures::Future;
use tokio_core::reactor::Core;
use tokio_service::Service;
use service_fn::service_fn;
use std::thread;
use std::time::Duration;
use tarantool::async_client::AsyncClient;

fn main() {

    let mut core = Core::new().unwrap();

    // This brings up our server.
    let addr = "127.0.0.1:3301".parse().unwrap();

    let handle = core.handle();

    core.run(
        AsyncClient::connect(&addr, &handle)
            .and_then(|client| {
                client.call(vec![])
                    .and_then(move |response| {
                        println!("CLIENT: {:?}", response);
                        client.call(vec![])
                    })
            })
    ).unwrap();

}