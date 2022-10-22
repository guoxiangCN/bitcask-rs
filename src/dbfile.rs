use crate::errors::{from_io_error, DBResult};
use std::{fs::File, io::ErrorKind, os::unix::prelude::FileExt};

use crate::model::{OwnedEntry, RefEntry};

type FileId = u64;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct EntryHandle {
    pub(crate) file_id: FileId,
    pub(crate) offset: u64,
    pub(crate) length: u64,
}

pub(crate) struct LogFile {
    id: FileId,
    file: File,
    offset: u64, // write posistion
}

impl LogFile {
    pub fn new(id: FileId, file: File) -> LogFile {
        LogFile {
            id: id,
            file: file,
            offset: 0,
        }
    }

    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    pub fn sync(&self) -> std::io::Result<()> {
        self.file.sync_all()
    }

    /// write_entry may write half-success and half-failure
    pub fn write_entry(&mut self, entry: RefEntry) -> DBResult<EntryHandle> {
        let data = entry.encode_to_bytes();
        assert!(data.len() > 0);

        let origin_offset = self.offset;
        let mut nwrite = 0;

        while nwrite < data.len() {
            let bytes = match self.file.write_at(&data[nwrite..], self.offset) {
                Ok(bytes) => bytes,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Err(from_io_error(e)),
            };
            if bytes == 0 {
                break; // EOF
            }
            self.offset += bytes as u64;
            nwrite += bytes;
        }
        Ok(EntryHandle {
            file_id: self.id,
            offset: origin_offset,
            length: self.offset - origin_offset,
        })
    }

    pub fn read_entry(&self, handle: EntryHandle) -> DBResult<OwnedEntry> {
        assert!(self.id == handle.file_id);
        let mut buf = vec![0_u8; handle.length as usize];
        let nread = match self.file.read_at(buf.as_mut(), handle.offset) {
            Ok(bytes) => bytes,
            Err(e) => return Err(from_io_error(e)),
        };

        if nread as u64 != handle.length {
            todo!("");
        }
        OwnedEntry::decode_from_bytes(&buf, false)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{OpType, OwnedEntry};

    use super::LogFile;

    #[test]
    fn test_write_read() {
        let f = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .read(true)
            .open("/tmp/00000000001")
            .unwrap();
        let mut dbf = LogFile::new(1, f);
        let mut oe = OwnedEntry {
            op_type: crate::model::OpType::Put,
            key: Vec::from("name"),
            value: Some(Vec::from("guoxiang")),
            ts: Some(100000000000003),
        };
        let re = oe.as_ref_entry();
        let data = re.encode_to_bytes();
        let handle = dbf.write_entry(re).unwrap();
        dbf.sync().unwrap();

        assert!(handle.length == data.len() as u64);
        println!("{:?}", handle);

        let read_entry = dbf.read_entry(handle.clone()).unwrap();
        assert_eq!(read_entry, oe);
        println!("{:?}", read_entry);

        // test del
        let last_offset = dbf.get_offset();
        oe.op_type = OpType::Del;
        oe.key = Vec::from("name");
        oe.value = None;
        oe.ts = Some(100000000000004);

        let re = oe.as_ref_entry();
        let data = re.encode_to_bytes();
        let handle = dbf.write_entry(re).unwrap();
        dbf.sync().unwrap();

        assert!(handle.length == data.len() as u64);
        assert!(handle.offset == last_offset);
        println!("{:?}", handle);
        let read_entry = dbf.read_entry(handle.clone()).unwrap();
        assert_eq!(read_entry, oe);
        println!("{:?}", read_entry);
    }
}
