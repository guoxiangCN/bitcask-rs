pub struct Options {
    pub target_file_size: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            target_file_size: 32 * 1024 * 1024,
        }
    }
}

pub struct ReadOptions {
    pub verify_checksum: bool,
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self {
            verify_checksum: false,
        }
    }
}

pub struct WriteOptions {
    pub sync: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self { sync: false }
    }
}
