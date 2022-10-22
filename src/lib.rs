#[allow(dead_code)]
mod db;
mod dbfile;
mod errors;
mod filename;
mod model;
mod options;
mod writebatch;

pub use db::BitcaskDB;
pub use writebatch::WriteBatch;
pub use options::{Options, ReadOptions, WriteOptions};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
       
    }
}
