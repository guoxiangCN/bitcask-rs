use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::dbfile::{EntryBlock, EntryHandle, FileId, KeyAndEntryHandle, LogFile, INVALID_FILE_ID};
use crate::errors::{from_io_error, DBError, DBResult};
use crate::filename::FileType;
use crate::model::OpType;
use crate::options::{Options, ReadOptions, WriteOptions};
use crate::versionset::{VersionEdit, VersionSet};
use crate::writebatch::WriteBatch;

pub struct BitcaskDB {
    options: Arc<Options>,
    core: Arc<Mutex<BitcaskCore>>,
}

struct BitcaskCore {
    mut_log: Option<Rc<LogFile>>,
    imm_logs: HashMap<FileId, Rc<LogFile>>,
    mem_index: BTreeMap<Vec<u8>, EntryHandle>,
    row_cache: HashMap<EntryHandle, EntryBlock>,

    bg_error: Option<DBError>,
    version_set: VersionSet,
    path: PathBuf,
}

impl BitcaskCore {
    fn new(dbpath: PathBuf) -> Self {
        Self {
            mut_log: None,
            imm_logs: HashMap::new(),
            mem_index: BTreeMap::new(),
            row_cache: HashMap::new(),
            bg_error: None,
            version_set: VersionSet::new(dbpath.clone()),
            path: dbpath.clone(),
        }
    }

    fn prepare_new_mutlog(&mut self) -> DBResult<Rc<LogFile>> {
        let new_log_id = self.version_set.new_logfile_id();
        let new_log_path = FileType::Log.get_full_filepath(self.path.clone(), new_log_id);
        let file = std::fs::File::options()
            .append(true)
            .create(true)
            .read(true)
            .write(true)
            .open(new_log_path)
            .map_err(|e| from_io_error(e))?;
        let logfile = Rc::new(LogFile::new(new_log_id, file));
        let mut edit = VersionEdit::default();
        edit.new_mut = Some(new_log_id);
        edit.mut_to_imm = self.mut_log.as_ref().map(|x| x.get_file_id());
        self.version_set.log_and_apply(&edit)?;
        self.mut_log.replace(logfile.clone());
        Ok(logfile.clone())
    }

    fn remove_obsolete_files(&self) {
        todo!()
    }
}

impl BitcaskDB {
    pub fn open<P: AsRef<Path>>(path: P, options: Options) -> DBResult<BitcaskDB> {
        let db = BitcaskDB {
            options: Arc::new(options.clone()),
            core: Arc::new(Mutex::new(BitcaskCore::new(path.as_ref().to_path_buf()))),
        };
        Ok(db)
    }

    pub fn put(&self, options: WriteOptions, key: &[u8], value: &[u8]) -> DBResult<()> {
        let mut batch = WriteBatch::new();
        batch.put(key, value);
        self.write(options, &batch)
    }

    pub fn delete(&self, options: WriteOptions, key: &[u8]) -> DBResult<()> {
        let mut batch = WriteBatch::new();
        batch.delete(key);
        self.write(options, &batch)
    }

    pub fn write(&self, options: WriteOptions, batch: &WriteBatch) -> DBResult<()> {
        let mut core = self.core.lock().unwrap();
        let mut_log = match core.mut_log.clone() {
            Some(x) if x.get_offset() < self.options.target_file_size => x.clone(),
            _ => match core.prepare_new_mutlog() {
                Ok(f) => f.clone(),
                Err(e) => return Err(e),
            },
        };
        let handles = batch.consume_by(|x| {
            mut_log.write_entry(x).map(|h| KeyAndEntryHandle {
                key: x.key.clone(),
                handle: h,
            })
        })?;

        if options.sync {
            mut_log.sync()?;
            // TODO record bg error ?
        }

        for h in handles {
            core.mem_index.insert(h.key, h.handle);
        }
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

// impl EntryConsumer for BTreeMap<Vec<u8>, EntryHandle> {
//     fn consume(&mut self, entry: OwnedEntry) {
//         match entry.op_type {
//             OpType::Put => self.insert(entry.key, entry.value.unwrap()),
//             OpType::Del => self.remove(entry.key.as_ref()),
//         };
//     }
// }
