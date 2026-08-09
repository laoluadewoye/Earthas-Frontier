use crate::elements::{EFVersion, EFComponent, EFByteRep, EFByteRepBuilder, EFByteRepCompatible, EFByteRepCompatibleEnum};
use crate::elements::efid::{EFIDEntityOrName, EFQuery, EFResponse};
use crate::elements::common::string::EFString;
use crate::utils::result::{EFOk, EFError};
use crate::utils::versions::*;
use crate::utils::component_str::*;
use crate::utils::generic_vector::get_index_from_generic_vector;
use crate::utils::byte_vector::{get_byte_vectors_and_version_from_byte_rep, get_byte_rep_from_builder};

pub mod identity;
pub mod secret;

use super::primitives::unsigned_int::EFUSize;
use super::entity::{EFStaticEntityTracker, EFByteEntityTracker};
use identity::EFIdentity;
use secret::EFSecret;

#[derive(Debug)]
pub struct EFHook;

#[derive(Debug)]
pub struct EFReference;

#[derive(Debug)]
pub struct EFConnection;

#[derive(Debug)]
pub struct EFDataflow;

#[derive(Debug)]
pub struct EFCache;

#[derive(Debug)]
pub struct EFRule;

#[derive(Debug)]
pub struct EFTags;

pub struct EFSystem {
    id_salt: EFUSize,
    absolute_path: String,
    identities: EFStaticEntityTracker<EFIdentity>,
    secrets: EFStaticEntityTracker<EFSecret>,
    hooks: EFStaticEntityTracker<EFHook>,
    references: EFStaticEntityTracker<EFReference>,
    connections: EFStaticEntityTracker<EFConnection>,
    dataflows: EFStaticEntityTracker<EFDataflow>,
    cache: EFCache,
    byte_entities: EFByteEntityTracker,
    rules: EFStaticEntityTracker<EFRule>,
    tags: EFStaticEntityTracker<EFTags>,
}

pub struct EFGlobalState;
