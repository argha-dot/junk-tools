use std::fmt;

#[derive(PartialEq, Eq)]
pub enum DownloadChapterError {
    ChaterMayNotExist,
    CannotCreateFolder(String),
    CouldntRemoveFile(String),
    CouldntParseRegex(String),
}

impl fmt::Display for DownloadChapterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl fmt::Debug for DownloadChapterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
