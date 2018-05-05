pub mod ephemeral;
pub mod s3;

use std::fmt::Debug;

// Identifier is an opaque type used to represent the id that is used
// to locate a piece of data.
//
pub type Identifier = String;

// Description is a structure that contains at least the identifier
// for the piece of data that is used in the put and get calls.
//
pub struct Description {
    pub id: Identifier,
}

// Data is the type that is used to shuttle data around the different
// sides of the Arrakis servers. It could be streaming or in memory.
//
pub type Data = String;

// Status represents the error type that was encountered when running
// the query against the backend.
//
#[derive(Debug)]
pub enum Status {
    NotFound,
    InternalError,
}

// Error is a structure that holds an exceptional status and a
// description for now.
//
#[derive(Debug)]
pub struct Error {
    status: Status,
    description: String,
}

impl Error {
    pub fn new(status: Status, description: String) -> Error {
        Error {
            status: status,
            description: description,
        }
    }

    pub fn new_with_status(status: Status) -> Error {
        Self::new(status, "".to_string())
    }
}

pub trait Backend : Debug {
    fn list(&self) -> Result<Vec<Description>, Error>;
    fn put(&mut self, id: Option<Identifier>, data: Data)
           -> Result<Identifier, Error>;
    fn get(&self, id: Identifier)
           -> Result<&Data, Error>;
}
