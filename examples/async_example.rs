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
use tarantool::action_type::ActionType;

fn main() {

    let mut core = Core::new().unwrap();

    let handle = core.handle();

    core.run(AsyncClient::auth("127.0.0.1:3301", "test", "test", &handle).and_then(|mut client| {
            client.call(ActionType::Insert(Insert {
                space: 512,
                keys: vec![Value::from(111), Value::from("ROLLING STONES"), Value::from(2025)]
            })).and_then(|result| {
                println!("Insert result: {:?}", result);
                Ok(())
            });
        client.call(ActionType::Insert(Insert {
            space: 512,
            keys: vec![Value::from(1221), Value::from("ROLLING STONES"), Value::from(2025)]
        })).and_then(|result| {
            println!("Insert result: {:?}", result);
            Ok(())
        })
        }))
        .unwrap();

}
