use std::collections::{HashMap};

use uuid::{Uuid};

use backends::*;

#[derive(Debug)]
pub struct EphemeralBackend {
    data: HashMap<Identifier, Data>,
}

impl EphemeralBackend {
    pub fn new() -> EphemeralBackend {
        EphemeralBackend {
            data: HashMap::new(),
        }
    }

    fn generate_id() -> Identifier {
        format!("{}", Uuid::new_v4())
    }

    fn key_to_description(key: Identifier) -> Description {
        Description {
            id: key,
        }
    }
}

impl Backend for EphemeralBackend {
    fn list(&self) -> Result<Vec<Description>, Error> {
        Ok(self.data.keys().cloned()
            .map(|x| { Self::key_to_description(x) })
            .collect())
    }

    fn put(&mut self, id: Option<Identifier>, data: Data)
           -> Result<Identifier, Error> {
        let gen_id = id.unwrap_or_else(|| {
            Self::generate_id()
        });

        self.data.insert(gen_id.clone(), data);
        Ok(gen_id)
    }

    fn get(&self, id: Identifier)
           -> Result<&Data, Error> {
        match self.data.get(&id) {
            Some(obj) => Ok(obj),
            None => Err(Error::new_with_status(Status::NotFound))
        }
    }
}
