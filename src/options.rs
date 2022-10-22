use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Options {
    pub create_if_missing: bool,
    pub error_if_exists: bool,
    pub target_file_size: u64,
    pub row_cache_size: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            create_if_missing: true,
            error_if_exists: false,
            target_file_size: 32 * 1024 * 1024,
            row_cache_size: 0, // disable row cache
        }
    }
}

impl AsRef<Self> for Options {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct ReadOptions {
    pub verify_checksum: bool,
    pub fill_cache: bool,
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self {
            verify_checksum: false,
            fill_cache: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WriteOptions {
    pub sync: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self { sync: false }
    }
}
