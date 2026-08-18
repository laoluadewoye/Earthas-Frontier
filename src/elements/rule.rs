use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum EFRuleEffect {
    Allow,
    Deny
}

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

pub struct EFRuleTracker {
    rule_set: HashSet<String>,
    user_id_map: HashMap<String, Vec<String>>
}
