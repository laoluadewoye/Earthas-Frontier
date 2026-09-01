use crate::utils::result::{EFResult, EFError};
use crate::elements::uri::EFURIString;

pub trait EFPrivilege {
    fn to_string(&self) -> &str;
    fn get_all_as_strings() -> Vec<String>;
}

#[derive(Debug)]
pub enum EFRuleEffect {
    Allow,
    Deny
}

#[derive(Debug)]
pub enum EFRuleHashTarget {
    SingleHash(String),
    MultipleHash(Vec<String>)
}

#[derive(Debug)]
pub enum EFRuleIdTarget {
    SingleId(EFURIString),
    MultipleId(Vec<EFURIString>),
}

#[derive(Debug)]
pub enum EFRulePropertyTarget<P: EFPrivilege> {
    All,
    AllForEffect(EFRuleEffect),
    AllForPrivilege(P),
    AllForId(String),
}

#[derive(Debug)]
pub enum EFRuleTarget<P: EFPrivilege> {
    Hash(EFRuleHashTarget),
    Id(EFRuleIdTarget),
    All(EFRulePropertyTarget<P>)
}

#[derive(Debug)]
pub struct EFRule<P: EFPrivilege> {
    id: String,
    effect: EFRuleEffect,
    privilege: P
}

#[derive(Debug)]
pub struct EFAnonRule<P: EFPrivilege> {
    effect: EFRuleEffect,
    privilege: P
}

pub trait EFRuleTracker {
    type PrivilegeType: EFPrivilege;

    fn new() -> Self;

    // Checking the contents of the rule tracker
    fn get_rule_count() -> EFResult<usize>;
    fn get_hashes() -> EFResult<EFRuleHashTarget>;
    fn get_ids() -> EFResult<Vec<EFURIString>>;
    fn get_privileges() -> EFResult<Vec<String>>;
    
    // Getting rules
    fn get_rules_by_hashes(hash_target: &EFRuleHashTarget) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>;
    fn get_rules_by_property(
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>;

    // Adding rules
    fn add_rules(
        id_target: &EFRuleIdTarget,
        rules: &Vec<EFAnonRule<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;

    // Popping rules
    fn pop_rules_by_hashes(hash_target: &EFRuleHashTarget) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>;
    fn pop_rules_by_property(
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>;

    // Set comparisons
    fn union_of(
        targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn intersection_of(
        targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn not_intersection_of(
        targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn only_in_first(
        first_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>,
        second_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn is_subset_of_first(
        first_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>,
        second_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<bool>;
}

pub enum EFRuleTrackerRequest<P: EFPrivilege> {
    GetRuleCount,
    GetHashes,
    GetIds,
    GetPrivileges,
    GetRules(EFRuleTarget<P>),
    AddRules(Vec<EFRuleTarget<P>>),
    PopRules(EFRuleTarget<P>),
    GetUnion(Vec<EFRuleTarget<P>>),
    GetIntersection(Vec<EFRuleTarget<P>>),
    GetNotIntersection(Vec<EFRuleTarget<P>>),
    GetOnlyInFirst(Vec<EFRuleTarget<P>>, Vec<EFRuleTarget<P>>),
    GetIsSubsetOfFirst(Vec<EFRuleTarget<P>>, Vec<EFRuleTarget<P>>)
}

pub enum EFRuleTrackerResponse<P: EFPrivilege> {
    Int(usize),
    Hashes(EFRuleHashTarget),
    Ids(EFURIString),
    Privileges(Vec<String>),
    Rules(Vec<EFRule<P>>),
    Bool(bool),
    Error(EFError)
}

/*
You should be able to manipulate a rule by-

1) It's content-addressable hash
2) The identifier or privilege type

hash = hash_fun(id, priv, effect)

the tracker should have
A vector of rules
A hashmap of hashes to rules
A hashmap of effets to rules
A hashmap of ids to rules
A hashmap of privileges to rules
*/

#[derive(Debug, Clone)]
pub struct EFBasicRuleTracker<P: EFPrivilege>;
