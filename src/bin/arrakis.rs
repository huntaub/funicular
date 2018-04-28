extern crate arrakis;

extern crate rusoto_core;

use arrakis::client;

use rusoto_core::{Region};

fn main() {
    println!("{:?}",
             client::BackendClient::new("ajidamal-data".to_string(), Region::UsEast1).list());
}
