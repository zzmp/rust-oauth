#![crate_id = "oauth"]

#![feature(phase)]

#[phase(syntax, link)] extern crate log;

extern crate http;
extern crate url;

pub mod server;
