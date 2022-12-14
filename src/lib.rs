#![feature(cell_update)]

#[allow(dead_code)]

mod db;
mod cache;
mod dbfile;
mod errors;
mod filename;
mod model;
mod options;
mod versionset;
mod writebatch;

pub use db::BitcaskDB;
pub use options::{Options, ReadOptions, WriteOptions};
pub use writebatch::WriteBatch;

#[cfg(test)]
mod tests {
    use crate::{BitcaskDB, Options, ReadOptions, WriteOptions};

    #[test]
    fn it_works() {
        let opts = Options::default();
        let bitcask = BitcaskDB::open("/tmp/bitcask001", opts).unwrap();
        let value = bitcask.get(ReadOptions::default(), b"__name__");
        assert!(value.is_ok());
        assert!(value.unwrap().is_none());

        // test put
        bitcask
            .put(WriteOptions::default(), b"name", b"guoxiang")
            .unwrap();
    }
}
