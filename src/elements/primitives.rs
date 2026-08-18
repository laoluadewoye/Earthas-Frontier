// Heavy use statements to bring functionality into scope
use crate::elements::{EFComponent, EFVersion, EFComponentTuple};
use crate::elements::uri::{EFQuery, EFResponse};
use crate::elements::byte_rep::{EFByteRep, EFByteRepBuilder, EFByteRepCompatible};
use crate::utils::result::{EFOk, EFError};
use crate::utils::versions::*;
use crate::utils::component_str::*;
use crate::utils::byte_vector::{
    get_byte_rep_from_builder, 
    get_byte_vectors_and_version_from_byte_rep
};
use crate::utils::generic_vector::get_index_from_generic_vector;

pub mod unsigned_int;
pub mod signed_int;
pub mod float;
pub mod boolean;
pub mod character;
