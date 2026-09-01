use super::*;

#[derive(Debug)]
pub struct EFStaticEntity<T: EFComponent> {
    id: EFEntityId,
    name: EFEntityName,
    owner: EFURIString,
    system: EFURIString,
    date_created: EFUTCTimestamp,
    date_accessed: EFUTCTimestamp,
    date_modified: EFUTCTimestamp,
    rules: EFBasicRuleTracker<EFEntityPrivilege>,
    files: EFBasicFileTracker,
    component: T,
}

impl<T: EFComponent> EFEntity for EFStaticEntity<T> {

}
