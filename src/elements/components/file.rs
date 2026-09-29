use super::*;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use regex::Regex;
use std::ops::Range;

#[derive(Debug)]
pub struct EFFilePath(String);

impl EFFilePath {
    pub fn new_if_valid(new_file_path: &String) -> EFValueResult<EFFilePath> {
        let is_valid: bool = new_file_path.chars().all(|c: char| {
            c.is_alphanumeric() 
                || c == '_' 
                || c == '-' 
                || c == '$' 
                || c == '/'
                || c == '.'
        });

        if is_valid {
            Ok(EFFilePath(new_file_path.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "new_if_valid", 
                "New file path is not valid."
            ))
        }
    }
}

pub struct EFSnapshot {
    file_path: EFFilePath,
    retention: EFTimePeriod
}

/* Cannot find a good encryption crate to support right now, table the encryption. */
#[derive(Debug)]
pub struct EFFile {
    name: EFName,
    file_path: EFFilePath,
    snapshots: HashMap<String, EFSnapshot>,
    byte_size: usize,
    checksum: EFId
}

#[derive(Debug)]
pub struct EFFileObject(File);

#[derive(Debug)]
pub struct EFFileBytes(Vec<u8>);

#[derive(Debug)]
pub struct EFFileString(String);

#[derive(Debug)]
pub struct EFFileLines(Vec<String>);

#[derive(Debug)]
pub struct EFFileTrackerEntry {
    name: EFName,
    uri: EFURIString,
    is_deleted: bool
}

pub trait EFFileTracker {
    fn new() -> Self;

    // File tracker metadata
    fn get_file_count(&self) -> usize;
    fn get_file_names(&self) -> Vec<EFName>;
    fn get_file_names_from_regex(
        &self, 
        file_name_re: &Regex
    ) -> EFValueResult<Vec<EFName>>;

    // File creation
    fn create(&mut self, file_name: &String) -> EFResult<()>;
    fn create_from_bytes(
        &mut self, 
        file_name: &String, 
        file_bytes: &EFFileBytes
    ) -> EFResult<()>;
    fn create_from_string(
        &mut self, 
        file_name: &String, 
        file_string: &EFFileString
    ) -> EFResult<()>;
    fn create_from_lines(
        &mut self, 
        file_name: &String, 
        file_lines: &EFFileLines
    ) -> EFResult<()>;

    // File data retrival
    fn get_file_obj(&self, file_name: &String) -> EFResult<EFFileObject>;
    fn get_file_obj_with_ops(
        &self, 
        file_name: &String, 
        options: &OpenOptions
    ) -> EFResult<EFFileObject>;
    fn get_bytes(&self, file_name: &String) -> EFResult<EFFileBytes>;
    fn get_string(&self, file_name: &String) -> EFResult<EFFileString>;
    fn get_lines(&self, file_name: &String) -> EFResult<EFFileLines>;

    // File data write
    fn write_bytes(
        &mut self, 
        file_name: &String, 
        file_bytes: &EFFileBytes,
        create_snapshot: bool
    ) -> EFResult<()>;
    fn write_string(
        &mut self, 
        file_name: &String, 
        file_string: &EFFileString,
        create_snapshot: bool
    ) -> EFResult<()>;
    fn write_lines(
        &mut self, 
        file_name: &String, 
        file_lines: &EFFileLines,
        create_snapshot: bool
    ) -> EFResult<()>;

    // File data replace
    fn overwrite_bytes_from_offset(
        &mut self, 
        file_name: &String, 
        new_bytes: &EFFileBytes,
        create_snapshot: bool,
        file_byte_offset: usize
    ) -> EFResult<()>;
    fn replace_byte_sequence(
        &mut self, 
        file_name: &String, 
        new_bytes: &EFFileBytes,
        create_snapshot: bool,
        old_byte_sequence: &EFFileBytes
    ) -> EFResult<()>;
    fn overwrite_string_from_index(
        &mut self, 
        file_name: &String, 
        new_string: &EFFileString,
        create_snapshot: bool,
        file_string_index: usize
    ) -> EFResult<()>;
    fn replace_string(
        &mut self, 
        file_name: &String, 
        new_string: &EFFileString,
        create_snapshot: bool,
        old_string: &EFFileString
    ) -> EFResult<()>;
    fn replace_line(
        &mut self, 
        file_name: &String, 
        new_line: &EFFileString,
        create_snapshot: bool,
        file_line_index: usize
    ) -> EFResult<()>;
    fn replace_multiple_lines(
        &mut self, 
        file_name: &String, 
        new_lines: &EFFileLines,
        create_snapshot: bool,
        file_line_index_range: Range<usize>
    ) -> EFResult<()>;

    // File data insert
    fn insert_bytes_at_offset(
        &mut self, 
        file_name: &String, 
        new_bytes: &EFFileBytes,
        create_snapshot: bool,
        file_byte_offset: usize
    ) -> EFResult<()>;
    fn insert_string_at_index(
        &mut self, 
        file_name: &String, 
        new_string: &EFFileString,
        create_snapshot: bool,
        file_string_index: usize
    ) -> EFResult<()>;
    fn insert_line(
        &mut self, 
        file_name: &String, 
        new_line: &EFFileString,
        create_snapshot: bool,
        file_line_index: usize
    ) -> EFResult<()>;
    fn insert_multiple_lines(
        &mut self, 
        file_name: &String, 
        new_lines: &EFFileLines,
        create_snapshot: bool,
        file_line_index: usize
    ) -> EFResult<()>;
    
    // File data append
    fn append_bytes(
        &mut self, 
        file_name: &String, 
        new_bytes: &EFFileBytes,
        create_snapshot: bool
    ) -> EFResult<()>;
    fn append_string(
        &mut self, 
        file_name: &String, 
        new_string: &EFFileString,
        create_snapshot: bool
    ) -> EFResult<()>;
    fn append_lines(
        &mut self, 
        file_name: &String, 
        new_lines: &EFFileLines,
        create_snapshot: bool
    ) -> EFResult<()>;

    // File data search
    fn find_byte_sequence(
        &self, 
        file_name: &String, 
        byte_sequence: &EFFileBytes
    ) -> EFResult<Vec<usize>>;
    fn find_string_regex(
        &self, 
        file_name: &String, 
        string_regex: &Regex
    ) -> EFResult<Vec<usize>>;

    // File deletion
    fn lazy_delete(&mut self, file_name: &String) -> EFResult<()>; // Requires compaction implementation on 
    fn eager_delete(&mut self, file_name: &String) -> EFResult<EFFileBytes>;
    fn restore(&mut self, file_name: &String) -> EFResult<()>; // Only works on lazy delete
    fn compact_tracker(&mut self) -> EFResult<()>;

    // Adds a snapshot to tracking
    fn create_snapshot(
        &mut self, 
        file_name: &String, 
        snapshot_name: &String, 
        snapshot_retention: &EFTimePeriod
    ) -> EFResult<()>;

    // Creates an entirely new entity
    fn create_backup(
        &mut self, 
        file_name: &String, 
        backup_name: &String
    ) -> EFResult<EFURIString>;
}

#[derive(Debug)]
pub enum EFFileData {
    None,
    Object(EFFileObject),
    Bytes(EFFileBytes),
    String(EFFileString),
    Lines(EFFileLines)
}

#[derive(Debug)]
pub enum EFFileDataType {
    Object,
    Bytes,
    String,
    Lines
}

#[derive(Debug)]
pub enum EFFileTrackerRequest {
    GetFileCount,
    GetFileNames,
    GetFileNamesRegex(Regex),
    Create(String, EFFileData),
    Get(String, EFFileDataType, Option<OpenOptions>),
    Write(String, EFFileData, bool),
    OverwriteBytes(String, EFFileBytes, bool, usize),
    ReplaceBytes(String, EFFileBytes, bool, EFFileBytes),
    OverwriteString(String, EFFileString, bool, usize),
    ReplaceString(String, EFFileString, bool, EFFileString),
    ReplaceLine(String, EFFileString, bool, usize),
    ReplaceMultipleLines(String, EFFileString, bool, Range<usize>),
    InsertBytes(String, EFFileBytes, bool, usize),
    InsertString(String, EFFileString, bool, usize),
    InsertLine(String, EFFileString, bool, usize),
    InsertMultipleLines(String, EFFileLines, bool, usize),
    Append(String, EFFileData, bool),
    FindBytes(String, EFFileBytes),
    FindString(String, Regex),
    LazyDelete(String),
    EagerDelete(String),
    Restore(String),
    CreateSnapshot(String, String, EFTimePeriod),
    CreateBackup(String, String)
}

#[derive(Debug)]
pub enum EFFileTrackerResponse {
    Count(usize),
    Names(Vec<EFName>),
    Data(EFFileData),
    FoundIndicies(Vec<usize>),
    Backup(EFURIString),
    Success(EFReturnEvent),
    Error(EFReturnEvent),
}