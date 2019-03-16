#![feature(proc_macro_hygiene)]

use tower_grpc_include::proto_include;

#[proto_include(path = "./hello_world.proto")]
mod hello_world {}
