use std::path::Path;

use crate::errors::DBResult;
use crate::options::{Options, ReadOptions, WriteOptions};
use crate::writebatch::WriteBatch;

pub struct BitcaskDB {}

impl BitcaskDB {
    fn new<P: AsRef<Path>>(path: P, options: Options) -> DBResult<BitcaskDB> {
        Ok(BitcaskDB {})
    }

    pub fn put(&self, options: WriteOptions, key: &[u8], value: &[u8]) -> DBResult<()> {
        Ok(())
    }

    pub fn delete(&self, options: WriteOptions, key: &[u8]) -> DBResult<()> {
        Ok(())
    }

    pub fn write(&self, options: WriteOptions, batch: &WriteBatch) -> DBResult<()> {
        Ok(())
    }

    pub fn get(&self, options: ReadOptions, key: &[u8]) -> DBResult<Option<Vec<u8>>> {
        Ok(None)
    }

    pub fn flush_all(&self) -> DBResult<()> {
        Ok(())
    }
}
