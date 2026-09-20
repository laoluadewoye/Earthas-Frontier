// Heavy use statements to bring functionality into scope
use crate::elements::{EFComponent, EFVersion};
use crate::elements::uri::{EFRequest, EFResponse};
use crate::utils::result::*;
use crate::utils::element_versions::*;
use crate::utils::element_types::*;
use crate::utils::vector::get_generic_vec_obj;

// Note: No need to wrap primitives, you can implement traits directly
pub mod unsigned_int;
pub mod signed_int;
pub mod float;
pub mod boolean;
pub mod character;
