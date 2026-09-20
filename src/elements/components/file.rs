use crate::utils::result::*;
use std::fs::{File, OpenOptions};
use regex::Regex;
use std::ops::Range;

/*
 New File Object attributes

 Note: This might warrant upgrading the file tracker to file being an entity component.

 - File path
 - Name
 - Type extension
 - Validity Period (Permenant or Temporary with Expiration Date)
 - Retention Period (Permenant or Temporary with Expiration Date)
 - Encryption Status
 - Version Tracking
 - Last Created, Accessed, Modified (Create a struct that contains these things for repeated use)
 - File Size
 - Checksum
*/

#[derive(Debug)]
pub struct EFFileName(String);

pub fn valid_filename(filename: String) -> bool {
    filename.chars().all(|c: char | c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
}

#[derive(Debug)]
pub struct EFFile(File);

#[derive(Debug)]
pub struct EFFileBytes(Vec<u8>);

#[derive(Debug)]
pub struct EFFileLines(Vec<String>);

pub trait EFFileTracker {
    fn new() -> Self;

    // Checking the contents of the file tracker
    fn get_file_count() -> EFResult<usize>;
    fn get_file_names() -> EFResult<Vec<EFFileName>>;
    fn get_file_names_from_regex(file_name_re: &Regex) -> EFResult<Vec<EFFileName>>;

    // Do some basic file interactions
    fn create_file(filename: &String) -> EFResult<EFFileName>;
    fn read_file(file: &EFFileName) -> EFResult<EFFileBytes>;
    fn write_to_file(
        file: &EFFileName,
        contents: &EFFileBytes
    ) -> EFResult<EFSuccess>;
    fn append_to_file(
        file: &EFFileName, 
        contents: &EFFileBytes
    ) -> EFResult<EFSuccess>;
    fn insert_to_file_line(
        file: &EFFileName, 
        lines: &EFFileLines, 
        target: usize
    ) -> EFResult<EFSuccess>;
    fn replace_in_file(
        file: &EFFileName, 
        old: &String,
        new: &String
    ) -> EFResult<EFSuccess>;
    fn replace_lines_in_file(
        file: &EFFileName, 
        lines: &EFFileLines, 
        target_range: Range<usize>
    ) -> EFResult<EFSuccess>;
    fn pop_file(file: &EFFileName) -> EFResult<EFFileBytes>;

    // Get file object
    fn get_file_object(
        file: &EFFileName, 
        options: &OpenOptions
    ) -> EFResult<EFFile>;
    fn get_multiple_file_objects(
        file: &Vec<EFFileName>, 
        options: &OpenOptions
    ) -> EFResult<Vec<EFFile>>;
}

pub enum EFFileTrackerRequest {
    GetFileCount,
    GetFileNames,
    GetFileNamesFromRegex(Regex),
    CreateFile(String),
    ReadFile(EFFileName),
    WriteToFile(EFFileName, EFFileBytes),
    AppendToFile(EFFileName, EFFileBytes),
    InsertToFileLine(EFFileName, EFFileLines, usize),
    ReplaceInFile(EFFileName, String, String),
    ReplaceLinesInFile(EFFileName, EFFileLines, Range<usize>),
    PopFile(EFFileName),
    GetFileObject(EFFileName, OpenOptions),
    GetMultipleFileObjects(Vec<EFFileName>, OpenOptions)
}

pub enum EFFileTrackerResponse {
    Int(usize),
    File(EFFile),
    FileNames(Vec<EFFileName>),
    FileBytes(Vec<EFFileBytes>),
    Success,
    Error(EFError)
}

#[derive(Debug, Clone)]
pub struct EFBasicFileTracker;
