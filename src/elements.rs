use crate::utils::result::{EFValueResult, EFReturnEvent};
use std::collections::HashMap;

#[derive(Debug)]
pub struct EFTuple {
    version: Vec<usize>,
    attributes: HashMap<String, String>
}

pub mod uri;
pub mod formats;
pub mod timestamp;
pub mod compatible;
