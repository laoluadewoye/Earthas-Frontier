use super::*;

#[derive(Debug)]
pub struct EFStaticEntity<T: EFComponent> {
    id: EFId,
    name: EFName,
    owner: EFURIString,
    system: EFURIString,
    time_metadata: EFTimeMetadata,
    rules: EFBasicRuleTracker<EFEntityPrivilege>,
    files: EFBasicFileTracker,
    tags: EFBasicTagTracker,
    component: T,
}

impl<T: EFComponent> EFEntity for EFStaticEntity<T> {
    
}
