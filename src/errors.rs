#[derive(Debug)]
pub struct DBError {}
pub type DBResult<T> = std::result::Result<T, DBError>;

pub(crate) fn from_io_error(e: std::io::Error) -> DBError {
    DBError {}
}
