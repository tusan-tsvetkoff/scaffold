use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FILE_REGEX: Regex = Regex::new(r"^(?P<name>.+?)(?P<ext>\..+)?$").unwrap();
    static ref DIRECTORY_REGEX: Regex = Regex::new(r"(?P<name>(/?[^/]+)*/)?$").unwrap();
    static ref DIRECTORY_FILE_REGEX: Regex =
        Regex::new(r"^(?P<dir>(?P<last>/?[^/]+)*/)(?P<file>.+?)(?P<ext>\..+)?$").unwrap();
}

pub enum ValidInput {
    File,
    Directory,
    DirectoryFile,
}

impl ValidInput {
    pub fn validate(input: &str) -> Option<Self> {
        if FILE_REGEX.is_match(input) {
            Some(ValidInput::File)
        } else if DIRECTORY_REGEX.is_match(input) {
            Some(ValidInput::Directory)
        } else if DIRECTORY_FILE_REGEX.is_match(input) {
            Some(ValidInput::DirectoryFile)
        } else {
            None
        }
    }
}
