use crate::elements::{EFVersion, EFComponent, EFComponentTuple};
use crate::elements::timestamp::EFUTCTimestamp;
use crate::elements::byte_rep::*;
use crate::elements::uri::{EFURIString, EFQuery, EFResponse};
use crate::elements::common::string::EFString;
use crate::utils::result::{EFOk, EFError};
use crate::utils::component_versions::*;
use crate::utils::component_types::*;
use crate::utils::vector::{get_index_from_generic_vector, get_string_from_byte_vector};

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
use super::entity::{EFStaticEntityTracker, EFByteEntityTracker};
use identity::EFIdentity;
use role::{EFRole};
use secret::EFSecret;
use hook::EFHook;
use reference::EFReference;
use connection::EFConnection;
use dataflow::EFDataflow;
use system_rule::EFSystemRule;
use tag::EFTag;

pub struct EFSystem {
    id_salt: EFUSize,
    absolute_path: EFString,
    identities: EFStaticEntityTracker<EFIdentity>,
    roles: EFStaticEntityTracker<EFRole>,
    secrets: EFStaticEntityTracker<EFSecret>,
    hooks: EFStaticEntityTracker<EFHook>,
    references: EFStaticEntityTracker<EFReference>,
    connections: EFStaticEntityTracker<EFConnection>,
    dataflows: EFStaticEntityTracker<EFDataflow>,
    byte_entities: EFByteEntityTracker,
    system_rules: EFStaticEntityTracker<EFSystemRule>,
    tags: EFStaticEntityTracker<EFTag>,
}

pub struct EFGlobalState;
