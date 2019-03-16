#![feature(proc_macro_hygiene)]

use tower_grpc_include::proto_include;

#[proto_include(proto = "proto/hello_world.proto", include = "proto")]
mod hello_world {}
