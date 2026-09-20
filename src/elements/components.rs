use crate::elements::{EFVersion, EFComponent};
use crate::elements::timestamp::EFUTCTimestamp;
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
pub mod tag;

use super::primitives::unsigned_int::EFUSize;
// use super::entity::{EFStaticEntityTracker, EFByteEntityTracker};
use identity::EFIdentity;
use role::{EFRole};
use secret::EFSecret;
use hook::EFHook;
use reference::EFReference;
use connection::EFConnection;
use dataflow::EFDataflow;
use system_rule::EFSystemRule;
use tag::EFTag;

#[derive(Debug)]
pub enum EFSystemPrivilege {
    SeeSystem,
    SeeSystemMetadata,
    SeeSystemEntities,
    SeeSubsystems,
    SeeParentSystem,
    ModifySystem,
    CreateEntities,
    DeleteEntities,
    DeleteSystem
}

// pub struct EFSystem {
//     id_salt: EFUSize,
//     absolute_path: EFString,
//     identities: EFStaticEntityTracker<EFIdentity>,
//     roles: EFStaticEntityTracker<EFRole>,
//     secrets: EFStaticEntityTracker<EFSecret>,
//     hooks: EFStaticEntityTracker<EFHook>,
//     references: EFStaticEntityTracker<EFReference>,
//     connections: EFStaticEntityTracker<EFConnection>,
//     dataflows: EFStaticEntityTracker<EFDataflow>,
//     byte_entities: EFByteEntityTracker,
//     system_rules: EFStaticEntityTracker<EFSystemRule>,
//     tags: EFStaticEntityTracker<EFTag>,
// }

pub struct EFGlobalState;
