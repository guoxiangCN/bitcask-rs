use std::{io::Write, path::PathBuf, sync::Arc};

use crate::dbfile::{FileId, INVALID_FILE_ID};
use crate::errors::{from_io_error, DBResult};
use crate::filename::FileType;

pub(crate) struct VersionSet {
    dbpath: PathBuf,
    next_logfile_id: FileId,
    manifest_file_id: FileId,
    current: Arc<Version>,
}

pub(crate) struct Version {
    pub(crate) mut_id: FileId,
    pub(crate) imm_ids: Vec<FileId>,
    pub(crate) manifest_id: FileId,
    // prev: Option<Arc<Version>>,
    // next: Option<Arc<Version>>,
}

#[derive(Debug, Default)]
pub(crate) struct VersionEdit {
    pub(crate) new_mut: Option<FileId>,
    pub(crate) mut_to_imm: Option<FileId>,
    pub(crate) compact_input_imm: Option<Vec<FileId>>,
    pub(crate) compact_output_imm: Option<Vec<FileId>>,
}

impl VersionSet {
    pub(crate) fn new(dbpath: PathBuf) -> Self {
        // Self {
        //     dbpath: dbpath,
        //     next_logfile_id: INVALID_FILE_ID + 1,
        //     manifest_file_id: 0,
        // }
        todo!()
    }

    pub(crate) fn new_logfile_id(&mut self) -> FileId {
        let last = self.next_logfile_id;
        self.next_logfile_id += 1;
        last
    }

    pub fn recovery(&self, save_manifest: bool) -> DBResult<()> {
        Ok(())
    }

    pub(crate) fn current(&self) -> Arc<Version> {
        // self.dbpath.join(path)
        todo!()
    }
    // Apply *edit to the current version to form a new descriptor that
    // is both saved to persistent state and installed as the new
    // current version.  Will release *mu while actually writing to the file.
    // REQUIRES: *mu is held on entry.
    // REQUIRES: no other thread concurrently calls LogAndApply()
    pub fn log_and_apply(&self, _edit: &VersionEdit) -> DBResult<()> {
        Ok(())
    }

    ///  Recover the last saved descriptor from persistent storage.

    fn write_current_file(&self, manifest_id: FileId) -> DBResult<()> {
        let manifest_file = FileType::Manifest.get_filename(manifest_id);
        let contents_to_write = manifest_file.to_str().unwrap();
        let current_filename =
            FileType::Manifest.get_full_filepath(self.dbpath.clone(), 0 /* not used */);
        let mut c = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(current_filename)
            .map_err(|e| from_io_error(e))?;

        c.write_all(contents_to_write.as_bytes())
            .map_err(|e| crate::errors::from_io_error(e))?;
        c.sync_all().map_err(|e| from_io_error(e))?;
        Ok(())
    }
}
