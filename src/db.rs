use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::dbfile::{EntryBlock, EntryHandle, FileId, LogFile, INVALID_FILE_ID};
use crate::errors::{DBError, DBResult};
use crate::model::OpType;
use crate::options::{Options, ReadOptions, WriteOptions};
use crate::writebatch::WriteBatch;

pub struct BitcaskDB {
    options: Arc<Options>,
    path: PathBuf,
    core: Arc<Mutex<BitcaskCore>>,
}

struct BitcaskCore {
    mut_log: Option<Arc<LogFile>>,
    imm_logs: HashMap<FileId, Arc<LogFile>>,
    mem_index: BTreeMap<Vec<u8>, EntryHandle>,
    row_cache: HashMap<EntryHandle, EntryBlock>,
    bg_error: Option<DBError>,
}

impl BitcaskCore {
    fn new() -> Self {
        Self {
            mut_log: None,
            imm_logs: HashMap::new(),
            mem_index: BTreeMap::new(),
            row_cache: HashMap::new(),
            bg_error: None,
        }
    }
}

impl BitcaskDB {
    pub fn open<P: AsRef<Path>>(path: P, options: Options) -> DBResult<BitcaskDB> {
        Ok(BitcaskDB {
            options: Arc::new(options.clone()),
            path: path.as_ref().to_path_buf(),
            core: Arc::new(Mutex::new(BitcaskCore::new())),
        })
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
        let core = self.core.lock().unwrap();
        let handle = match core.mem_index.get(key) {
            None => return Ok(None),
            Some(handle) => handle,
        };

        assert!(handle.file_id != INVALID_FILE_ID);
        let search_target_fn = || match core
            .mut_log
            .clone()
            .filter(|x| x.get_file_id() == handle.file_id)
        {
            Some(x) => return Some(x),
            None => {
                return match core.imm_logs.get(&handle.file_id) {
                    Some(x) => Some(x.clone()),
                    None => None,
                }
            }
        };
        let file = search_target_fn().expect("the index of key points to a non-exist place");
        let entry = file.read_entry(handle.clone(), options.verify_checksum)?;
        match entry.op_type {
            OpType::Put => return Ok(entry.value),
            OpType::Del => return Ok(None),
        }
    }

    pub fn flush_all(&self) -> DBResult<()> {
        todo!()
    }
}
