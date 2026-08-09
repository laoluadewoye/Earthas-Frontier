// Heavy use statements to bring functionality into scope
use crate::elements::{EFByteRep, EFByteRepBuilder, EFComponent, EFByteRepCompatible, EFVersion};
use crate::elements::efid::{EFQuery, EFResponse};
use crate::utils::result::{EFOk, EFError};
use crate::utils::versions::*;
use crate::utils::component_str::*;
use crate::utils::byte_vector::{
    get_byte_rep_from_builder, 
    get_byte_vectors_and_version_from_byte_rep,
    get_string_from_byte_vector
};
use crate::utils::generic_vector::get_index_from_generic_vector;

pub mod string;
