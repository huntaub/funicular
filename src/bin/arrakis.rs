extern crate arrakis;

extern crate rusoto_core;

use arrakis::backends::s3;

use rusoto_core::{Region};

fn main() {
    println!("{:?}",
             s3::BackendClient::new("ajidamal-data".to_string(), Region::UsEast1).list());
}
