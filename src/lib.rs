#![deny(warnings)]

extern crate rusoto_core;
extern crate rusoto_s3;

extern crate uuid;

extern crate futures;
extern crate hyper;

pub mod backends;
pub mod server;
