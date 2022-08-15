// Copyright (C) 2021, Achiefs.

// To manage unique event identifier
use uuid::Uuid;
// To get own process ID
use std::process;
// To get Operating system
use std::env;
// To use files IO operations.
use std::fs::File;
use std::io::{Read, SeekFrom};
use std::io::prelude::*;
// To get config constants
use crate::config;
// To manage paths
use std::path::Path;

// ----------------------------------------------------------------------------

// Function to pop last char of a given String
pub fn pop(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str()
}

// ----------------------------------------------------------------------------

pub fn get_hostname() -> String {
    gethostname::gethostname().into_string().unwrap()
}

// ----------------------------------------------------------------------------

pub fn get_uuid() -> String {
    format!("{}", Uuid::new_v4())
}

// ----------------------------------------------------------------------------

pub fn get_pid() -> u32 {
    process::id()
}

// ----------------------------------------------------------------------------

pub fn get_os() -> String {
    env::consts::OS.to_string()
}

// ----------------------------------------------------------------------------

// Function to read file from begin to end
pub fn read_file(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    contents
}

// ----------------------------------------------------------------------------

// (Only supported in Linux) Function to get machine id of the host
pub fn get_machine_id() -> String {
    read_file(String::from(config::MACHINE_ID_PATH))
}

// ----------------------------------------------------------------------------

// Function to get file name of a given path
pub fn get_filename_path(path: &str) -> String {
    String::from(Path::new(path).file_name().unwrap().to_str().unwrap())
}

// ----------------------------------------------------------------------------

// Function to clean trailing slash of a path
pub fn clean_path(path: &str) -> String {
    String::from(if path.ends_with('/') || path.ends_with('\\'){ pop(path) }else{ path })
}

// ----------------------------------------------------------------------------

// Function to get the last byte of a given file
pub fn get_file_end(file: &str) -> u64 {
    let mut f = File::open(file).unwrap();
    f.seek(SeekFrom::End(0)).unwrap()
}

// ----------------------------------------------------------------------------

// Function to determine if a String ends with given char or not
pub fn ends_with(string: &str, end: char) -> bool {
    String::from(string).pop().unwrap() == end
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        assert_eq!(pop("test-"), "test");
        assert_eq!(pop("dir/"), "dir");
        assert_eq!(pop("dir@"), "dir");
    }

}