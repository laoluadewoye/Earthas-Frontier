use crate::elements::timestamp::EFUTCTimestamp;
use crate::elements::uri::{EFURITarget, EFURIString};
use crate::utils::result::{EFError, EFOk, EFSuccess};
use crate::elements::rule::{EFRuleEffect};

#[derive(Debug)]
pub enum EFEntityPrivilege {
    CloneEntity, // Cloning structs
    AccessMetadata, // Timestamps, name, ID, owner
    ModifyMetadata, // Timestamps, name, ID, owner
    AccessUserRules,
    AccessAllRules,
    ModifyRules,
    CreateDeleteFiles,
    AccessFile(String),
    ModifyFile(String),
    AccessComponent,
    ModifyComponent
}

#[derive(Debug)]
pub struct EFEntityRule {
    id: String,
    effect: EFRuleEffect,
    privilege: EFEntityPrivilege
}

#[derive(Debug)]
pub struct EFEntityGroupRule {
    ids: Vec<String>,
    effect: EFRuleEffect,
    privilege: EFEntityPrivilege
}

#[derive(Debug)]
pub struct EFEntityRuleSet {
    id: String,
    privilege_effects: Vec<(EFRuleEffect, EFEntityPrivilege)>
}

#[derive(Debug)]
pub struct EFEntityGroupRuleSet {
    ids: Vec<String>,
    privilege_effects: Vec<(EFRuleEffect, EFEntityPrivilege)>
}

#[derive(Debug)]
pub struct EFEntityFile(String);

pub trait EFEntity {
    type EntityType;

    // Create new entity
    fn new() -> Self::EntityType;

    // Clone entity
    fn clone_entity(&self) -> Self::EntityType;

    // Check entity rules for a certain action
    fn check_privilege(
        &self, 
        current_id: &String,
        target_privilege: EFEntityPrivilege
    ) -> Result<EFOk<EFSuccess>, EFError>;

    // Work with entity's ID
    fn get_id(&self, current_id: &String) -> Result<EFOk<&String>, EFError>;

    // Work with entity's name
    fn get_name(&self, current_id: &String) -> Result<EFOk<&String>, EFError>;
    fn set_name(
        &mut self, 
        current_id: &String,
        new_name: String
    ) -> Result<EFOk<EFSuccess>, EFError>;

    // Work with owner of entity
    fn get_owner(&self, current_id: &String) -> Result<EFOk<&EFURIString>, EFError>;
    fn set_owner(
        &mut self, 
        current_id: &String, 
        new_owner: EFURIString
    ) -> Result<EFOk<EFSuccess>, EFError>;

    // Work with creation date, last accessed date, and last modified date
    // Probably will be indirect modifications
    fn get_created(&self, current_id: &String) -> Result<EFOk<&EFUTCTimestamp>, EFError>;

    fn get_last_accessed(&self, current_id: &String) -> Result<EFOk<&EFUTCTimestamp>, EFError>;
    fn set_last_accessed(
        &mut self, 
        current_id: &String, 
        new_timestamp: EFUTCTimestamp
    ) -> Result<EFOk<EFSuccess>, EFError>;

    fn get_last_modified(&self, current_id: &String) -> Result<EFOk<&EFUTCTimestamp>, EFError>;
    fn set_last_modified(
        &mut self, 
        current_id: &String,
        new_timestamp: EFUTCTimestamp
    ) -> Result<EFOk<EFSuccess>, EFError>;

    // Get the entity's rules
    fn get_all_rules(&self, current_id: &String) -> Result<EFOk<Vec<EFEntityRule>>, EFError>;
    fn get_all_rules_for_id(
        &self, 
        current_id: &String,
        target_id: String
    ) -> Result<EFOk<Vec<EFEntityRule>>, EFError>;
    fn get_all_rules_for_privilege(
        &self, 
        current_id: &String, 
        target_privilege: EFEntityPrivilege
    ) -> Result<EFOk<Vec<EFEntityRule>>, EFError>;

    // Add rules to the entity
    fn add_rule(
        &mut self,
        current_id: &String,
        new_rule: EFEntityRule
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn add_multiple_rules(
        &mut self,
        current_id: &String,
        new_rules: Vec<EFEntityRule>
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn add_multiple_rules_for_id(
        &mut self,
        current_id: &String,
        new_rules: EFEntityRuleSet
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn add_rule_for_multiple_ids(
        &mut self,
        current_id: &String,
        new_group_rule: EFEntityGroupRule
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn add_multiple_rules_for_multiple_ids(
        &mut self,
        current_id: &String,
        new_group_rules: EFEntityGroupRuleSet
    ) -> Result<EFOk<EFSuccess>, EFError>;

    // Remove rules from the entity
    fn remove_rule(
        &mut self,
        current_id: &String,
        target_rule: EFEntityRule
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn remove_multiple_rules(
        &mut self,
        current_id: &String,
        target_rules: Vec<EFEntityRule>
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn remove_multiple_rules_for_id(
        &mut self,
        current_id: &String,
        target_rules: EFEntityRuleSet
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn remove_rule_for_multiple_ids(
        &mut self,
        current_id: &String,
        target_group_rule: EFEntityGroupRule
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn remove_multiple_rules_for_multiple_ids(
        &mut self,
        current_id: &String,
        target_group_rules: EFEntityGroupRuleSet
    ) -> Result<EFOk<EFSuccess>, EFError>;

    // NOT DONE WITH THIS
    // NOT DONE WITH THIS
    // NOT DONE WITH THIS

    // Work with entity's files
    fn create_file(
        &mut self,
        current_id: &String,
        new_file_path_name: String
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn get_file(
        &self,
        current_id: &String,
        path_name: &String
    ) -> Result<EFOk<EFEntityFile>, EFError>;
    fn get_mutable_file(
        &mut self,
        current_id: &String,
        path_name: &String
    ) -> Result<EFOk<EFEntityFile>, EFError>;
    fn append_to_file(
        &mut self,
        current_id: &String,
        path_name: &String,
        new_line: String
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn insert_to_file(
        &mut self,
        current_id: &String,
        path_name: &String,
        new_line: String,
        line_number: usize
    ) -> Result<EFOk<EFSuccess>, EFError>;
    fn replace_in_file();
    fn replace_range_in_file();
    fn update_file();
    fn delete_file();

    // Work with entity's component
    fn get_component();
    fn get_mutable_component();
    fn get_component_type(&self) -> String;
    fn query_component();

    // NOT DONE WITH THIS
    // NOT DONE WITH THIS
    // NOT DONE WITH THIS
}

pub trait EFEntityTracker {
    type EntityType;

    fn new() -> Self;
    fn add_entity(&mut self, entity: Self::EntityType) -> Option<Self::EntityType>;
    fn pop_entity(&mut self, entity_id: &str) -> Result<EFOk<Self::EntityType>, EFError>;
    fn get_entity(&self, entity_id: &str) -> Result<EFOk<&Self::EntityType>, EFError>;
    fn get_mut_entity(&mut self, entity_id: &str) -> Result<EFOk<&mut Self::EntityType>, EFError>;
}
