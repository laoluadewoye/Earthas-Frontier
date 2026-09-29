use std::hash::Hash;
use crate::elements::{
    EFVersion, 
    EFComponent, 
    EFComponentRequest, 
    EFComponentResponse, 
    EFName, 
    EFId
};
use crate::elements::timestamp::{EFUTCTimestamp, EFTimeMetadata, EFTimePeriod};
use crate::elements::uri::{EFURIString};
use crate::utils::result::{EFResult, EFValueResult};
use crate::elements::rule::{
    EFBasicRuleTracker, EFRuleEffect, EFRulePrivilege, EFRuleTracker, EFRuleTrackerRequest, EFRuleTrackerResponse
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum EFEntityPrivilege {
    Owner, // Supercedes all other privileges
    CloneEntity,
    GetMetadata, // time metadata, name, ID, owner, system, validity, retention
    SetMetadata, // time metadata, name, owner, validity, retention
    AccessUserRules,
    AccessRules,
    ModifyRules,
    AccessTags,
    ModifyTags,
    GetComponentMetadata,
    BorrowComponent,
    BorrowMutableComponent,
    CloneComponent,
    UseComponentHandler
}

impl EFRulePrivilege for EFEntityPrivilege {
    fn as_str(&self) -> &str {
        match self {
            EFEntityPrivilege::Owner => "Owner",
            EFEntityPrivilege::CloneEntity => "Clone Entity",
            EFEntityPrivilege::GetMetadata => "Get Metadata",
            EFEntityPrivilege::SetMetadata => "Set Metadata",
            EFEntityPrivilege::AccessUserRules => "Access User Rules",
            EFEntityPrivilege::AccessRules => "Access Rules",
            EFEntityPrivilege::ModifyRules => "Modify Rules",
            EFEntityPrivilege::AccessTags => "Access Tags",
            EFEntityPrivilege::ModifyTags => "Modify Tags",
            EFEntityPrivilege::GetComponentMetadata => "Get Component Metadata",
            EFEntityPrivilege::BorrowComponent => "Borrow Component",
            EFEntityPrivilege::BorrowMutableComponent => "Borrow Mutable Component",
            EFEntityPrivilege::CloneComponent => "CloneComponent",
            EFEntityPrivilege::UseComponentHandler => "Use Component Handler",
        }
    }

    fn get_all_as_strings() -> Vec<String> {
        vec![
            String::from("Owner"),
            String::from("Clone Entity"),
            String::from("Get Metadata"),
            String::from("Set Metadata"),
            String::from("Access User Rules"),
            String::from("Access Rules"),
            String::from("Modify Rules"),
            String::from("Access Tags"),
            String::from("Modify Tags"),
            String::from("Get Component Metadata"),
            String::from("Borrow Component"),
            String::from("Borrow Mutable Component"),
            String::from("Clone Component"),
            String::from("Use Component Handler"),
        ]
    }
}

pub trait EFEntity {
    type EntityType: EFEntity;
    type PrivilegeType: EFRulePrivilege;
    type RuleTrackerType: EFRuleTracker;
    type ComponentType: EFComponent;
    type ComponentRequestType: EFComponentRequest;
    type ComponentResponseType: EFComponentResponse;

    // Create new entity
    fn new(
        name: EFName, 
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
        target_privilege: Self::PrivilegeType
    ) -> EFValueResult<EFRuleEffect>;

    // Work with entity's ID
    fn get_id(&self, current_id: &EFURIString) -> EFValueResult<&EFId>;

    // Work with entity's name
    fn get_name(&self, current_id: &EFURIString) -> EFValueResult<&EFName>;
    fn set_name(
        &mut self, 
        current_id: &EFURIString,
        new_name: &String
    ) -> EFResult<&EFName>;

    // Work with owner of entity
    fn get_owner(&self, current_id: &EFURIString) -> EFValueResult<&EFURIString>;
    fn set_owner(
        &mut self, 
        current_id: &EFURIString, 
        new_owner: &EFURIString
    ) -> EFResult<&EFURIString>;

    // Work with entity's system
    fn get_system(&self, current_id: &EFURIString) -> EFValueResult<&EFURIString>;

    // Work with creation date, last accessed date, and last modified date
    fn get_created(&self, current_id: &EFURIString) -> EFValueResult<&EFUTCTimestamp>;

    fn get_last_accessed(&self, current_id: &EFURIString) -> EFValueResult<&EFUTCTimestamp>;
    fn set_last_accessed(
        &mut self, 
        current_id: &EFURIString, 
        new_timestamp: &EFUTCTimestamp
    ) -> EFResult<()>;

    fn get_last_modified(&self, current_id: &EFURIString) -> EFValueResult<&EFUTCTimestamp>;
    fn set_last_modified(
        &mut self, 
        current_id: &EFURIString,
        new_timestamp: &EFUTCTimestamp
    ) -> EFResult<()>;

    // Work with validity and retention periods
    fn get_validity_period(&self, current_id: &EFURIString) -> EFValueResult<&EFTimePeriod>;
    fn set_validity_period(
        &mut self, 
        current_id: &EFURIString,
        new_time_period: &EFTimePeriod
    ) -> EFResult<()>;

    fn get_retention_period(&self, current_id: &EFURIString) -> EFValueResult<&EFTimePeriod>;
    fn set_retention_period(
        &mut self, 
        current_id: &EFURIString,
        new_time_period: &EFTimePeriod
    ) -> EFResult<()>;

    // Work with entity's rules
    fn rule_tracker_action(
        &mut self, 
        current_id: &EFURIString,
        request: &EFRuleTrackerRequest<Self::PrivilegeType>
    ) -> EFResult<EFRuleTrackerResponse<Self::PrivilegeType>>;

    // Work with entity's tags
    // Tag list should be a hash set
    fn get_tags(&self, current_id: &EFURIString) -> EFResult<Vec<&EFURIString>>;
    fn check_for_tag(
        &self, 
        current_id: &EFURIString,
        tag_id: &EFURIString
    ) -> EFResult<bool>;
    fn add_tag(
        &mut self, 
        current_id: &EFURIString,
        new_tag_id: &EFURIString
    ) -> EFResult<()>;
    fn remove_tag(
        &mut self, 
        current_id: &EFURIString,
        old_tag_id: &EFURIString
    ) -> EFResult<()>;

    // Get component metadata
    fn get_component_version(&self, current_id: &EFURIString) -> EFResult<&EFVersion>;
    fn get_component_type(&self, current_id: &EFURIString) -> EFResult<&str>;

    // Borrow an entity's component
    fn borrow_component(&self, current_id: &EFURIString) -> EFValueResult<&Self::ComponentType>;
    fn borrow_mutable_component(
        &mut self,
        current_id: &EFURIString
    ) -> EFValueResult<&mut Self::ComponentType>;

    // Clone the component
    fn clone_component(&self, current_id: &EFURIString) -> EFValueResult<Self::ComponentType>;

    // Handle a component request
    fn handle_component_request(
        &self,
        current_id: &EFURIString,
        request: &Self::ComponentRequestType
    ) -> Self::ComponentResponseType;
}

pub trait EFEntityTracker {
    type EntityType: EFEntity;

    fn new() -> Self;
    fn get_entity_count(&self) -> usize;
    fn get_entity_names(&self) -> Vec<EFName>;
    fn get_entity_ids(&self) -> Vec<EFId>;
    fn get_entity_tags(&self) -> Vec<EFURIString>;

    fn add_entity(&self, new_entity: Self::EntityType) -> EFResult<&EFId>;
    fn get_entity_by_name(&self, entity_name: &String) -> EFResult<&Self::EntityType>;
    fn get_entity_by_id(&self, entity_id: &String) -> EFResult<&Self::EntityType>;
    fn get_entity_ids_by_tags(&self, entity_tags: &Vec<EFURIString>) -> EFResult<Vec<&EFId>>;
    fn get_mutable_entity_by_name(&mut self, entity_name: &String) -> EFResult<&mut Self::EntityType>;
    fn get_mutable_entity_by_id(&mut self, entity_id: &String) -> EFResult<&mut Self::EntityType>;
    fn pop_entity_by_name(&self, entity_name: &String) -> EFResult<Self::EntityType>;
    fn pop_entity_by_id(&self, entity_id: &String) -> EFResult<Self::EntityType>;
}

pub mod static_entity;
pub mod dynamic_entity;
