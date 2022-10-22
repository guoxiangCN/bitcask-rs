use crate::dbfile::FileId;

pub(crate) enum FileType {
    MutLog, // active file for write
    ImmLog, // immutable file for read only

    Hint,     // persisit index for speeding recover
    Manifest, // the manifest to manage the whole db and compaction
    Lock,     // lock file
    Current, // the current file points to the manifest used.
}

impl FileType {
    pub(crate) fn get_filename(&self, file_id: FileId) -> String {
        match self {
            FileType::MutLog => format!("{:09}.mut", file_id),
            FileType::ImmLog => format!("{:09}.imm", file_id),
            FileType::Hint => format!("{:09}.hit",file_id),
            FileType::Manifest => format!("MANIFEST-{:09}", file_id),
            FileType::Lock => "LOCK".to_owned(),
            FileType::Current => "CURRENT".to_owned(),
        }       
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;

    #[test]
    fn test_get_filename() {
        assert_eq!("LOCK", FileType::Lock.get_filename(0));
        assert_eq!("CURRENT", FileType::Current.get_filename(0));
        assert_eq!("MANIFEST-000000001", FileType::Manifest.get_filename(1));
        assert_eq!("000000002.imm", FileType::ImmLog.get_filename(2));
        assert_eq!("000000002.mut", FileType::MutLog.get_filename(2));
        assert_eq!("000000003.hit", FileType::Hint.get_filename(3));
    }
}