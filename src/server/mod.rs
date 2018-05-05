use std::net::{SocketAddr};
use std::sync::{Arc};

use futures::future;

// use hyper::header::Content Length;
use hyper::error::{Error};
use hyper::server::{Http, Request, Response, Service};

use backends::{Backend};

#[derive(Debug)]
pub struct Frontend {
    backend: Arc<Box<Backend>>,
}

impl Frontend {
    pub fn run(addr: &SocketAddr, backend: Box<Backend>) {
        let backend_arc = Arc::new(backend);
        Http::new().bind(addr, move || {
            Ok(Frontend {
                backend: backend_arc.clone(),
            })
        }).unwrap().run().unwrap()
    }
}

impl Service for Frontend {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<future::Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        // Create the HTTP response
        let resp = Response::new()
            .with_body("hello world\n");

        // Return the response as an immediate future
        Box::new(future::ok(resp))
    }
}
