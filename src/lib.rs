mod cache;
#[allow(dead_code)]
mod db;
mod dbfile;
mod errors;
mod filename;
mod model;
mod options;
mod writebatch;

pub use db::BitcaskDB;
pub use options::{Options, ReadOptions, WriteOptions};
pub use writebatch::WriteBatch;

#[cfg(test)]
mod tests {
    use crate::{BitcaskDB, Options, ReadOptions};

    #[test]
    fn it_works() {
        let opts = Options::default();
        let bitcask = BitcaskDB::open("/tmp/bitcask001", opts).unwrap();
        let value = bitcask.get(ReadOptions::default(),b"__name__");
        assert!(value.is_ok());
        assert!(value.unwrap().is_none());
    }
}
