use crate::elements::{EFVersion, EFComponent, EFName, EFId};
use crate::elements::timestamp::{EFUTCTimestamp, EFTimePeriod};
use crate::elements::uri::{EFURIString};
use crate::elements::common::string::EFString;
use crate::utils::result::*;
use crate::utils::element_versions::*;
use crate::utils::element_types::*;
use crate::utils::vector::{get_generic_vec_obj, get_str_slice_from_vec_u8};

pub mod identity;
pub mod role;
pub mod secret;
pub mod hook;
pub mod reference;
pub mod connection;
pub mod dataflow;
pub mod system_rule;
pub mod file;
