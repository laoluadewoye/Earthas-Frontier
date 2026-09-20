use std::collections::HashMap;
use crate::utils::result::{EFValueResult, EFReturnEvent};

pub mod uri;
pub mod primitives;
pub mod common;
pub mod entity;
pub mod components;
pub mod timestamp;
pub mod rule;
pub mod tracker;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EFVersion(pub usize, pub usize, pub usize);

impl EFVersion {
    pub fn get_major(&self) -> usize { self.0 }
    pub fn get_minor(&self) -> usize { self.1 }
    pub fn get_patch(&self) -> usize { self.2 }
}

#[derive(Debug)]
pub struct EFTuple {
    version: Vec<usize>,
    attributes: HashMap<String, String>
}

#[derive(Debug)]
pub struct EFName(String);

impl EFName {
    pub fn new_if_valid(new_name: &String) -> EFValueResult<EFName> {
        let is_valid: bool = new_name.chars().all(|c: char| {
            c.is_alphanumeric() || c == '_' || c == '-' || c == ' '
        });

        if is_valid {
            Ok(EFName(new_name.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "new_if_valid", 
                "New name is not valid."
            ))
        }
    }
}

#[derive(Debug)]
pub struct EFId(String);

impl EFId {
    pub fn new_if_valid(new_id: &String) -> EFValueResult<EFId> {
        let is_valid: bool = new_id.chars().all(|c: char| {
            c.is_ascii_hexdigit()
        });

        if is_valid {
            Ok(EFId(new_id.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "new_if_valid", 
                "New id is not valid."
            ))
        }
    }
}

pub trait EFComponentRequest {}
pub trait EFComponentResponse {}

pub trait EFComponent: Clone {
    type ComponentParams;
    type ComponentRequestType: EFComponentRequest;
    type ComponentResponseType: EFComponentResponse;
    
    // Create functions
    fn create_new(params: Self::ComponentParams) -> Self;
    fn create_from_compatible(params: Self::ComponentParams, version: EFVersion) -> Self;

    // Getter functions
    fn get_component_version(&self) -> &EFVersion;
    fn get_component_type(&self) -> &str;

    // Tuple functions
    fn from_older_tuple(tuple: EFTuple) -> Self;
    fn to_older_tuple(&self, older_version: EFVersion) -> EFTuple;

    // Query functions
    fn handle_request(&self, request: &Self::ComponentRequestType) -> Self::ComponentResponseType;
}
