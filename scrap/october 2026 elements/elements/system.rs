pub struct EFSystem {}

pub trait EFSystemElement {}

// use super::primitives::unsigned_int::EFUSize;
// use super::entity::{EFStaticEntityTracker, EFByteEntityTracker};
// use identity::EFIdentity;
// use role::{EFRole};
// use secret::EFSecret;
// use hook::EFHook;
// use reference::EFReference;
// use connection::EFConnection;
// use dataflow::EFDataflow;
// use system_rule::EFSystemRule;
// use tag::EFTag;
// use file::EFFile;

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
//     files: EFStaticEntityTracker<EFFile>,
// }