use std::path::Path;

use crate::chunk::Chunk;

use jammdb::{DB, Error};

pub struct Jammstore {
    db: DB,
}

impl Jammstore {
    pub fn new(path: &Path) -> Result<Jammstore, Error> {
        let db = DB::open(path)?;
        Ok(Jammstore{db: db})
    }

    pub fn put(mut self, chunk: Chunk) -> Result<(), Error> {
        let mut tx = self.db.tx(true)?;
        let mut bucket = tx.get_or_create_bucket("chunks")?;
        bucket.put(chunk.hash().sum, chunk.data())?;
        Ok(())
    }   
}
