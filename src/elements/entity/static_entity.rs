use super::*;

#[derive(Debug)]
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
}

impl<T: EFComponent> EFEntity for EFStaticEntity<T> {
    
}
