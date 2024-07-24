use std::fmt;

// Custom error type; can be any type which defined in the current crate
// In here, we use a simple "unit struct" to simplify the example
#[derive(PartialEq, Eq)]
pub enum DownloadChapterError {
    ChaterMayNotExist,
    CannotCreateFolder(String),
    CouldntRemoveFile(String),
    CouldntParseRegex(String),
}

// Implement std::fmt::Display for AppError
impl fmt::Display for DownloadChapterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for DownloadChapterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
