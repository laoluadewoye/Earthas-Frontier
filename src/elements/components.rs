pub mod identity;

use super::entity::{EFStaticEntityTracker, EFDynamicEntityTracker};
use super::primitives::unsigned_int::EFUSize;
use identity::EFIdentity;

#[derive(Debug)]
pub struct EFSecret {
    secret: String,
    salt: String,
    salt_generated_date: String
}

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
    entities: EFDynamicEntityTracker,
    rules: EFStaticEntityTracker<EFRule>,
    tags: EFStaticEntityTracker<EFTags>,
}

pub struct EFGlobalState;
