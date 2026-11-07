use super::*;

use crate::elements::rule::EFRuleTracker;
use crate::utils::general::get_hash;

#[derive(Debug, Clone)]
pub struct EFStaticEntity<T: EFComponent> {
    id: EFId,
    name: EFName,
    owner: EFURIString,
    system: EFURIString,
    time_metadata: EFTimeMetadata,
    validity: EFTimePeriod,
    retention: EFTimePeriod,
    rules: EFBasicRuleTracker<EFEntityPrivilege>,
    tags: Vec<EFURIString>,
    component: T,
    event: EFReturnEvent // Add stuff to the entity trait to account for this
}

impl<T: EFComponent> EFEntity for EFStaticEntity<T> {
    type PrivilegeType = EFEntityPrivilege;
    type ComponentType = T;

    fn new(
        name: EFName, 
        owner: EFURIString,
        system: EFURIString,
        validity: EFTimePeriod,
        retention: EFTimePeriod,
        component: Self::ComponentType,
        entity_hash: &String
    ) -> Self
    {
        let new_time_metadata: EFTimeMetadata = EFTimeMetadata::new();
        let salt: String = new_time_metadata.get_created().to_string();
        let hash_vec: Vec<&String> = vec![&name.0, &owner.0, &system.0, &salt];
        let new_id: EFId = match get_hash(hash_vec, entity_hash) {
            Ok(s) => match EFId::new_if_valid(&s) {
                Ok(i) => i,
                Err(e) => { return Self::fallback(e) }
            },
            Err(e) => { return Self::fallback(e) }
        };

        EFStaticEntity { 
            id: new_id, 
            name, 
            owner, 
            system, 
            time_metadata: new_time_metadata, 
            validity, 
            retention, 
            rules: EFBasicRuleTracker::new(), 
            tags: Vec::new(), 
            component,
            event: EFReturnEvent::new_with_func_info_log(
                "EFStaticEntity::new", 
                "Created new static entity."
            )
        }
    }

    fn fallback(event: EFReturnEvent) -> Self {
        EFStaticEntity { 
            id: EFId(String::new()), 
            name: EFName(String::new()), 
            owner: EFURIString(String::new()), 
            system: EFURIString(String::new()), 
            time_metadata: EFTimeMetadata::new(), 
            validity: EFTimePeriod::Forever, 
            retention: EFTimePeriod::Forever, 
            rules: EFBasicRuleTracker::new(), 
            tags: Vec::new(), 
            component: T::create_fallback(),
            event
        }
    }

    fn clone_entity(&self) -> Self {
        self.clone()
    }

    fn check_privilege(
        &self, 
        current_id: &EFURIString,
        target_privilege: &Self::PrivilegeType
    ) -> EFValueResult<EFRuleEffect>
    {
        // Check if owner
        
        // Look for allow rule

        // Look for deny rule
    }
}
