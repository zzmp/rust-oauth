#![crate_id = "oauth"]
#![crate_type="lib"]

//! A gated server request handler
//!
//! All requests will be passed through a basic oAuth 'gate'
//! before being allowed to see the request handler.

// This allows for debug macros
// Use the macros by setting your RUST_LOG environment variable appropriately
#![feature(phase)]
#[phase(syntax, link)] extern crate log;

extern crate http;
extern crate url;

pub mod server;
