extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate eui48;
extern crate ssdp;
extern crate futures;
extern crate hyper;
extern crate tokio_core;


mod discovery;
mod yamaha;
use std::env::Args;
use std::net::{IpAddr};

