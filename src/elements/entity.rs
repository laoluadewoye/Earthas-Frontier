pub mod static_entity;
pub mod dynamic_entity;

use crate::elements::*;
use crate::elements::file::*;
use crate::elements::timestamp::EFUTCTimestamp;
use crate::elements::uri::{EFURIString};
use crate::utils::result::*;
use crate::elements::rule::*;

#[derive(Debug)]
pub enum EFEntityPrivilege {
    Owner, // Supercedes all other privileges
    CloneEntity,
    GetMetadata, // Timestamps, name, ID, owner, system
    SetMetadata, // Timestamps, name, owner
    AccessUserRules, // Only see rules set for user
    AccessRuleTracker, 
    ModifyRuleTracker,
    AccessUserFiles, // Only see files made by user
    AccessFileTracker,
    ModifyFileTracker,
    AccessComponent,
    ModifyComponent,
    UseComponent // Use the handler API
}

impl EFPrivilege for EFEntityPrivilege {
    fn to_string(&self) -> &str {
        match self {
            EFEntityPrivilege::Owner => "Owner",
            EFEntityPrivilege::CloneEntity => "Clone Entity",
            EFEntityPrivilege::GetMetadata => "Get Metadata",
            EFEntityPrivilege::SetMetadata => "Set Metadata",
            EFEntityPrivilege::AccessUserRules => "Access User Rules",
            EFEntityPrivilege::AccessRuleTracker => "Access Rule Tracker",
            EFEntityPrivilege::ModifyRuleTracker => "Modify Rule Tracker",
            EFEntityPrivilege::AccessUserFiles => "Access User Files",
            EFEntityPrivilege::AccessFileTracker => "Access File Tracker",
            EFEntityPrivilege::ModifyFileTracker => "Modify File Tracker",
            EFEntityPrivilege::AccessComponent => "Access Component",
            EFEntityPrivilege::ModifyComponent => "Modify Component",
            EFEntityPrivilege::UseComponent => "Use Component"
        }
    }

    fn get_all_as_strings() -> Vec<String> {
        vec![
            String::from("Owner"),
            String::from("Clone Entity"),
            String::from("Get Metadata"),
            String::from("Set Metadata"),
            String::from("Access User Rules"),
            String::from("Access Rule Tracker"),
            String::from("Modify Rule Tracker"),
            String::from("Access User Files"),
            String::from("Access File Tracker"),
            String::from("Modify File Tracker"),
            String::from("Access Component"),
            String::from("Modify Component"),
            String::from("Use Component"),
        ]
    }
}

#[derive(Debug)]
pub struct EFEntityName(String);

pub fn valid_entity_name(entity_name: String) -> bool {
    entity_name.chars().all(|c: char | c.is_alphanumeric() || c == '_' || c == '-' || c == ' ')
}

#[derive(Debug)]
pub struct EFEntityId(String);

pub fn valid_entity_id(entity_id: String) -> bool {
    entity_id.chars().all(|c: char | c.is_ascii_hexdigit())
}

pub trait EFEntity {
    type EntityType: EFEntity;
    type ComponentType: EFComponent;
    type ComponentRequestType: EFComponentRequest;
    type ComponentResponseType: EFComponentResponse;

    // Create new entity
    fn new(
        name: EFEntityName, 
        owner: EFURIString,
        system: EFURIString,
        component: Self::ComponentType
    ) -> Self::EntityType;

    // Clone entity
    fn clone_entity(&self) -> Self::EntityType;

    // Check entity rules for a certain action
    fn check_privilege(
        &self, 
        current_id: &EFURIString,
        target_privilege: EFEntityPrivilege
    ) -> EFResult<EFSuccess>;

    // Work with entity's ID
    fn get_id(&self, current_id: &EFURIString) -> EFResult<&EFEntityId>;

    // Work with entity's name
    fn get_name(&self, current_id: &EFURIString) -> EFResult<&EFEntityName>;
    fn set_name(
        &mut self, 
        current_id: &EFURIString,
        new_name: EFEntityName
    ) -> EFResult<EFSuccess>;

    // Work with owner of entity
    fn get_owner(&self, current_id: &EFURIString) -> EFResult<&EFURIString>;
    fn set_owner(
        &mut self, 
        current_id: &EFURIString, 
        new_owner: EFURIString
    ) -> EFResult<EFSuccess>;

    // Work with entity's system
    fn get_system(&self, current_id: &EFURIString) -> EFResult<&EFURIString>;

    // Work with creation date, last accessed date, and last modified date
    fn get_last_created(&self, current_id: &EFURIString) -> EFResult<&EFUTCTimestamp>;

    fn get_last_accessed(&self, current_id: &EFURIString) -> EFResult<&EFUTCTimestamp>;
    fn set_last_accessed(
        &mut self, 
        current_id: &EFURIString, 
        new_timestamp: EFUTCTimestamp
    ) -> EFResult<EFSuccess>;

    fn get_last_modified(&self, current_id: &EFURIString) -> EFResult<&EFUTCTimestamp>;
    fn set_last_modified(
        &mut self, 
        current_id: &EFURIString,
        new_timestamp: EFUTCTimestamp
    ) -> EFResult<EFSuccess>;

    // Work with entity's rules
    fn rule_tracker_action(
        &mut self, 
        current_id: &EFURIString,
        request: &EFRuleTrackerRequest<EFEntityPrivilege>
    ) -> EFResult<EFRuleTrackerResponse<EFEntityPrivilege>>;

    // Work with entity's files
    fn file_tracker_action(
        &mut self, 
        current_id: &EFURIString,
        request: &EFFileTrackerRequest
    ) -> EFResult<EFFileTrackerResponse>;

    // Work with entity's component
    fn borrow_component(&self, current_id: &EFURIString) -> EFResult<&Self::ComponentType>;
    fn borrow_mutable_component(
        &mut self,
        current_id: &EFURIString
    ) -> EFResult<&mut Self::ComponentType>;
    fn clone_component(&self, current_id: &EFURIString) -> EFResult<Self::ComponentType>;
    fn get_component_as_older(
        &self, 
        current_id: &EFURIString, 
        old_version: &EFVersion
    ) -> EFResult<EFComponentTuple>;
    fn get_component_version(&self, current_id: &EFURIString) -> EFResult<&EFVersion>;
    fn get_component_type(&self, current_id: &EFURIString) -> EFResult<&str>;
    fn handle_component_request(
        &self,
        current_id: &EFURIString,
        request: &Self::ComponentRequestType
    ) -> Self::ComponentResponseType;
}

pub trait EFEntityTracker {
    type EntityType: EFEntity;

    fn new() -> Self;
    fn get_entity_count(&self) -> EFResult<usize>;
    fn get_entity_names(&self) -> EFResult<Vec<EFEntityName>>;
    fn get_entity_ids(&self) -> EFResult<Vec<EFEntityId>>;

    fn add_entity(&self, new_entity: Self::EntityType) -> EFResult<EFSuccess>;
    fn get_entity_by_name(&self, name: &EFEntityName) -> EFResult<&Self::EntityType>;
    fn get_entity_by_id(&self, id: &EFEntityId) -> EFResult<&Self::EntityType>;
    fn get_mutable_entity_by_name(&mut self, name: &EFEntityName) -> EFResult<&mut Self::EntityType>;
    fn get_mutable_entity_by_id(&mut self, id: &EFEntityId) -> EFResult<&mut Self::EntityType>;
    fn pop_entity_by_name(&self, name: &EFEntityName) -> EFResult<Self::EntityType>;
    fn pop_entity_by_id(&self, id: &EFEntityId) -> EFResult<Self::EntityType>;
}
