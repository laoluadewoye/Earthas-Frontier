use std::hash::Hash;
use std::collections::HashMap;
use crate::utils::result::*;
use crate::utils::general::get_hash;
use crate::elements::uri::EFURIString;

pub trait EFPrivilege: Clone + Eq + Hash {
    fn as_str(&self) -> &str;
    fn get_all_as_strings() -> Vec<String>;
}

#[derive(Debug, Clone)]
pub enum EFRuleEffect {
    Allow,
    Deny
}

impl EFRuleEffect {
    pub fn as_str(&self) -> &str {
        match self {
            EFRuleEffect::Allow => "Allow",
            EFRuleEffect::Deny => "Deny"
        }
    }
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
    AllForId(EFURIString),
}

#[derive(Debug)]
pub enum EFRuleTarget<P: EFPrivilege> {
    Hash(EFRuleHashTarget),
    Id(EFRuleIdTarget),
    All(EFRulePropertyTarget<P>)
}

#[derive(Debug, Clone)]
pub struct EFRule<P: EFPrivilege> {
    id: EFURIString,
    effect: EFRuleEffect,
    privilege: P
}

impl<P: EFPrivilege> EFRule<P> {
    pub fn to_hash(&self, rule_hash: &String) -> EFResult<String> {
        get_hash(
            vec![
                &self.id.0, 
                &self.effect.as_str().to_string(), 
                &self.privilege.as_str().to_string()
            ], 
            rule_hash
        )
    }

    pub fn to_string_desc(&self) -> String {
        format!(
            "{} {} for {}", 
            self.id.0.as_str(),
            self.effect.as_str(),
            self.privilege.as_str()
        )
    }
}

#[derive(Debug)]
pub struct EFAnonRule<P: EFPrivilege> {
    effect: EFRuleEffect,
    privilege: P
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum EFRuleTrackerResponse<P: EFPrivilege> {
    Int(usize),
    Hashes(EFRuleHashTarget),
    Ids(EFURIString),
    Privileges(Vec<String>),
    Rules(Vec<EFRule<P>>),
    Bool(bool),
    Error(EFError)
}

pub trait EFRuleTracker {
    type PrivilegeType: EFPrivilege;

    fn new() -> Self;

    // Checking the contents of the rule tracker
    fn get_rule_count(&self) -> EFResult<usize>;
    fn get_hashes(&self) -> EFResult<EFRuleHashTarget>;
    fn get_ids(&self) -> EFResult<Vec<EFURIString>>;
    fn get_privileges(&self) -> EFResult<Vec<Self::PrivilegeType>>;
    
    // Getting rules
    fn get_rules_by_hashes(&self, hash_target: &EFRuleHashTarget) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>>;
    fn get_rules_by_property(
        &self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>>;

    // Adding rules
    fn add_rules(
        &mut self,
        id_target: &EFRuleIdTarget,
        anon_rules: &Vec<EFAnonRule<Self::PrivilegeType>>,
        rule_hash: &String
    ) -> EFResult<EFRuleHashTarget>;

    // Popping rules
    fn pop_rules_by_hashes(
        &mut self,
        hash_target: &EFRuleHashTarget
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>;
    fn pop_rules_by_property(
        &mut self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>;

    // Set comparisons
    fn union_of(
        &self,
        targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn intersection_of(
        &self,
        targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn not_intersection_of(
        &self,
        targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn only_in_first(
        &self,
        first_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>,
        second_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<EFRuleHashTarget>;
    fn is_subset_of_first(
        &self,
        first_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>,
        second_targets: &Vec<EFRuleTarget<Self::PrivilegeType>>
    ) -> EFResult<bool>;

    // Request handler
    fn handle_rule_tracker_request(
        &self,
        request: EFRuleTrackerRequest<Self::PrivilegeType>
    ) -> EFResult<EFRuleTrackerResponse<Self::PrivilegeType>>;
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

#[derive(Debug)]
pub struct EFBasicRuleTrackerHashesEntry {
    rules_index: usize,
    allow_rules_index: Option<usize>,
    deny_rules_index: Option<usize>,
    ids_index: usize,
    privileges_index: usize
}

#[derive(Debug)]
pub enum EFBasicRuleTrackerTryPopResult {
    OutOfBounds,
    Removed,
    RemovedAndEmpty
}

#[derive(Debug)]
pub struct EFBasicRuleTracker<P: EFPrivilege> {
    rules: Vec<EFRule<P>>,
    allow_rules: Vec<usize>,
    deny_rules: Vec<usize>,
    hashes: HashMap<String, EFBasicRuleTrackerHashesEntry>,
    ids: HashMap<EFURIString, Vec<usize>>,
    privileges: HashMap<P, Vec<usize>>
}

impl<P: EFPrivilege> EFBasicRuleTracker<P> {
    fn add_to_effect(
        &mut self, 
        rule_combo_effect: &EFRuleEffect, 
        rule_index: usize
    ) -> (Option<usize>, Option<usize>) {
        let mut allow_rules_index: Option<usize> = None;
        let mut deny_rules_index: Option<usize> = None;
        if let EFRuleEffect::Allow = rule_combo_effect {
            allow_rules_index = Some(self.allow_rules.len());
            self.allow_rules.push(rule_index);
        }
        else {
            deny_rules_index = Some(self.deny_rules.len());
            self.deny_rules.push(rule_index);
        }

        (allow_rules_index, deny_rules_index)
    }

    fn add_to_ids(
        &mut self,
        rule_combo_id: &EFURIString,
        rule_index: usize
    ) -> usize {
        let ids_index: usize;
        match self.ids.get_mut(rule_combo_id) {
            Some(id_vec) => {
                ids_index = id_vec.len();
                id_vec.push(rule_index); 
            },
            None => {
                ids_index = 0;
                self.ids.insert(rule_combo_id.clone(), vec![rule_index]); 
            }
        }
        ids_index
    }

    fn add_to_privileges(
        &mut self,
        rule_combo_privilege: &P,
        rule_index: usize
    ) -> usize {
        let privileges_index: usize;
        match self.privileges.get_mut(rule_combo_privilege) {
            Some(priv_vec) => {
                privileges_index = priv_vec.len();
                priv_vec.push(rule_index);
            },
            None => {
                privileges_index = 0;
                self.privileges.insert(rule_combo_privilege.clone(), vec![rule_index]);
            }
        }
        privileges_index
    }

    fn add_to_tracker(&mut self, rule_hash: String, rule_combo: &EFRule<P>) {
        // Insert to rules
        let rules_index: usize = self.rules.len();
        self.rules.push(rule_combo.clone());

        // Insert to effect
        let (allow_rules_index, deny_rules_index) = self.add_to_effect(&rule_combo.effect, rules_index);

        // Insert to ids
        let ids_index: usize = self.add_to_ids(&rule_combo.id, rules_index);

        // Insert to privileges
        let privileges_index: usize = self.add_to_privileges(&rule_combo.privilege, rules_index);

        // Insert to hashes
        self.hashes.insert(
            rule_hash, 
            EFBasicRuleTrackerHashesEntry{
                rules_index,
                allow_rules_index, 
                deny_rules_index, 
                ids_index, 
                privileges_index 
            }
        );
    }

    fn try_pop_rules(&mut self, rule_index: usize) -> bool {
        // Check if there's an out of bounds possibility
        if rule_index < 0 || rule_index >= self.rules.len() {
            return false;
        }

        // Try to delete if it's in bounds
        self.rules.remove(rule_index);
        true
    }

    fn try_pop_tracker(tracker_vec: &mut Vec<usize>, rule_index: usize) -> EFBasicRuleTrackerTryPopResult {
        // Check if there's an out of bounds possibility
        if rule_index < 0 || rule_index >= tracker_vec.len() {
            return EFBasicRuleTrackerTryPopResult::OutOfBounds;
        }

        // Try to delete if it's in bounds
        tracker_vec.remove(rule_index);

        // Check if the vec is empty
        if !tracker_vec.is_empty() {
            EFBasicRuleTrackerTryPopResult::Removed
        }
        else {
            EFBasicRuleTrackerTryPopResult::RemovedAndEmpty
        }
    }

    // New problem: Implement a tombstone and compression functionality
    fn pop_from_tracker(&mut self, rule_hash: &String) -> EFResult<EFRule<P>> {        
        // Try to get a hash entry while deleting from tracker
        let rule_entry: EFBasicRuleTrackerHashesEntry = match self.hashes.remove(rule_hash) {
            Some(r_e) => r_e,
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.hashes.get(&rule_hash)"), 
                    msg: format!("Could not find hash for {}.", rule_hash.as_str())
                });
            }
        };
        let removed_rule: &EFRule<P> = &self.rules[rule_entry.rules_index];

        // Delete from rules
        self.try_pop_rules(rule_entry.rules_index);

        // Delete from allow rules
        if let Some(allow_rules_index) = rule_entry.allow_rules_index {
            EFBasicRuleTracker::<P>::try_pop_tracker(&mut self.allow_rules, allow_rules_index);
        }

        // Delete from deny rules
        if let Some(deny_rules_index) = rule_entry.deny_rules_index {
            EFBasicRuleTracker::<P>::try_pop_tracker(&mut self.deny_rules, deny_rules_index);
        }

        // Delete from ids
        let ids_pop_result: EFBasicRuleTrackerTryPopResult = match self.ids.get_mut(&removed_rule.id) {
            Some(id_vec) => EFBasicRuleTracker::<P>::try_pop_tracker(
                id_vec, rule_entry.ids_index
            ),
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.ids.get_mut(&removed_rule.id)"), 
                    msg: format!("ID {} does not have any rules.", removed_rule.id.0.as_str())
                });
            }
        };
        if let EFBasicRuleTrackerTryPopResult::RemovedAndEmpty = ids_pop_result {
            self.ids.remove(&removed_rule.id);
        }

        // Delete from privileges
        let privileges_pop_result: EFBasicRuleTrackerTryPopResult = match self.privileges.get_mut(&removed_rule.privilege) {
            Some(privilege_vec) => EFBasicRuleTracker::<P>::try_pop_tracker(
                privilege_vec, rule_entry.privileges_index
            ),
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.privileges.get_mut(&removed_rule.privilege)"), 
                    msg: format!("Privilege {} does not have any rules.", removed_rule.privilege.as_str())
                });
            }
        };
        if let EFBasicRuleTrackerTryPopResult::RemovedAndEmpty = privileges_pop_result {
            self.privileges.remove(&removed_rule.privilege);
        }

        // Return the rule
        Ok(EFOk{
            value: removed_rule,
            msg: format!("Popped rule {} from tracker.", rule_hash.as_str())
        })
    }
}

impl<P: EFPrivilege> EFRuleTracker for EFBasicRuleTracker<P> {
    type PrivilegeType = P;

    fn new() -> Self {
        EFBasicRuleTracker {
            rules: Vec::new(),
            allow_rules: Vec::new(), 
            deny_rules: Vec::new(), 
            hashes: HashMap::new(), 
            ids: HashMap::new(), 
            privileges: HashMap::new()
        }
    }

    fn get_rule_count(&self) -> EFResult<usize> {
        Ok(EFOk{
            value: self.rules.len(),
            msg: String::from("Got length of rule tracker.")
        })
    }

    fn get_hashes(&self) -> EFResult<EFRuleHashTarget> {
        let hash_vec: Vec<String> = self.hashes.keys().into_iter().map(|k| k.clone()).collect();
        Ok(EFOk{
            value: EFRuleHashTarget::MultipleHash(hash_vec),
            msg: String::from("Got list of rule hashes.")
        })
    }

    fn get_ids(&self) -> EFResult<Vec<EFURIString>> {
        let id_vec: Vec<EFURIString> = self.ids.keys().into_iter().map(|k| k.clone()).collect();
        Ok(EFOk{
            value: id_vec,
            msg: String::from("Got list of ids with rules.")
        })
    }

    fn get_privileges(&self) -> EFResult<Vec<Self::PrivilegeType>> {
        let privilege_vec: Vec<Self::PrivilegeType> = self.privileges.keys().into_iter().map(|k| k.clone()).collect();
        Ok(EFOk{
            value: privilege_vec,
            msg: String::from("Got list of privileges with rules.")
        })
    }

    fn get_rules_by_hashes(&self, hash_target: &EFRuleHashTarget) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>> {
        match hash_target {
            EFRuleHashTarget::SingleHash(s) => match self.hashes.get(s) {
                Some(r) => Ok(EFOk{
                    value: vec![&r.rule], 
                    msg: format!("Found hash for {}.", s.as_str())
                }),
                None => Err(EFError{
                    function: String::from("get_rules_by_hashes"), 
                    line: String::from("self.hashes.get(s)"), 
                    msg: format!("Could not find hash for {}.", s.as_str())
                })
            },
            EFRuleHashTarget::MultipleHash(v_s) => {
                let mut found_rules: Vec<&EFRule<Self::PrivilegeType>> = Vec::new();
                let mut missed_hashes: Vec<&String> = Vec::new();

                for hash in v_s {
                    match self.hashes.get(hash) {
                        Some(r) => { found_rules.push(&r.rule); },
                        None => { missed_hashes.push(hash); }
                    }
                }

                if found_rules.len() == 0 {
                    Err(EFError{
                        function: String::from("get_rules_by_hashes"), 
                        line: String::from("found_rules.len() == 0"), 
                        msg: String::from("Could not find any hash in list.")
                    })
                }
                else if missed_hashes.len() == 0 {
                    Ok(EFOk{ value: found_rules, msg: String::from("Found all hashes.") })
                }
                else {
                    let mut missed_hashes_str: String = String::from("Found some hashes. Missed hashes were ");
                    for missed_hash in missed_hashes {
                        missed_hashes_str.push_str(missed_hash.as_str());
                        missed_hashes_str.push_str(", ");
                    }
                    missed_hashes_str.push('.');

                    Ok(EFOk{ value: found_rules, msg: missed_hashes_str })
                }
            }
        }
    }

    fn get_rules_by_property(
        &self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>>
    {
        match property_target {
            EFRulePropertyTarget::All => Ok(EFOk{
                value: self.rules.iter().map(|r| r).collect(), 
                msg: String::from("Got all rules.")
            }),
            EFRulePropertyTarget::AllForEffect(r) => match r {
                EFRuleEffect::Allow => Ok(EFOk{
                    value: self.allow_rules.iter().map(|r| *r).collect(), 
                    msg: String::from("Got all allow rules.")
                }),
                EFRuleEffect::Deny => Ok(EFOk{
                    value: self.deny_rules.iter().map(|r| *r).collect(), 
                    msg: String::from("Got all deny rules.")
                })
            },
            EFRulePropertyTarget::AllForPrivilege(p) => match self.privileges.get(p) {
                Some(p_vec) => Ok(EFOk{
                    value: p_vec.iter().map(|r| *r).collect(), 
                    msg: format!("Got rules for {}.", p.as_str())
                }),
                None => Err(EFError{
                    function: String::from("get_rules_by_property"),
                    line: String::from("self.privileges.get(p)"),
                    msg: format!("Could not get rules for {}.", p.as_str())
                })
            },
            EFRulePropertyTarget::AllForId(id) => match self.ids.get(id) {
                Some(id_vec) => Ok(EFOk{
                    value: id_vec.iter().map(|r| *r).collect(), 
                    msg: format!("Got rules for {}.", id.0.as_str())
                }),
                None => Err(EFError{
                    function: String::from("get_rules_by_property"),
                    line: String::from("self.ids.get(id)"),
                    msg: format!("Could not get rules for {}.", id.0.as_str())
                })
            }
        }
    }

    fn add_rules(
        &mut self,
        id_target: &EFRuleIdTarget,
        anon_rules: &Vec<EFAnonRule<Self::PrivilegeType>>,
        rule_hash: &String
    ) -> EFResult<EFRuleHashTarget>
    {
        // Create rule combinations
        let mut rule_combos: Vec<EFRule<Self::PrivilegeType>> = Vec::new();
        if let EFRuleIdTarget::SingleId(single_id) = id_target {
            for anon_rule in anon_rules {
                rule_combos.push(EFRule{
                    id: single_id.clone(),
                    effect: anon_rule.effect.clone(),
                    privilege: anon_rule.privilege.clone()
                });
            }
        }
        else if let EFRuleIdTarget::MultipleId(ids) = id_target {
            for single_id in ids {
                for anon_rule in anon_rules {
                    rule_combos.push(EFRule{
                        id: single_id.clone(),
                        effect: anon_rule.effect.clone(),
                        privilege: anon_rule.privilege.clone()
                    });
                }
            }
        }
        
        // Try adding to tracker
        let mut output_target: Vec<String> = Vec::new();
        let mut existing_rules: String = String::new();

        for rule_combo in rule_combos {
            let rule_combo_hash: String = match rule_combo.to_hash(rule_hash) {
                Ok(h) => h.value,
                Err(e) => { return Err(e); }
            };

            if let None = self.hashes.get(&rule_combo_hash) {
                output_target.push(rule_combo_hash.clone());
                self.add_to_tracker(rule_combo_hash, rule_combo);
            }
            else {
                existing_rules.push_str(rule_combo.to_string_desc().as_str());
                existing_rules.push('\n');
            }
        }

        // Return attempt information
        let output_target_len: usize = output_target.len();
        Ok(EFOk{
            value: EFRuleHashTarget::MultipleHash(output_target),
            msg: format!(
                "Added {} rules. The following rules could not be added:\n{}",
                output_target_len,
                existing_rules.as_str()
            )
        })
    }

    fn pop_rules_by_hashes(
        &mut self,
        hash_target: &EFRuleHashTarget
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>
    {   
        match hash_target {
            EFRuleHashTarget::SingleHash(s) => match self.pop_from_tracker(s) {
                Ok(r) => Ok(EFOk{
                    value: vec![r.value],
                    msg: format!("Popped rule for {}.", s.as_str())
                }),
                Err(e) => Err(e)
            },
            EFRuleHashTarget::MultipleHash(v_s) => {
                let mut popped_rules: Vec<EFRule<Self::PrivilegeType>> = Vec::new();
                let mut missed_hashes: Vec<&String> = Vec::new();

                for hash in v_s {
                    match self.pop_from_tracker(hash) {
                        Ok(r) => { popped_rules.push(r.value); },
                        Err(_) => { missed_hashes.push(hash); }
                    }
                }

                if popped_rules.len() == 0 {
                    Err(EFError{
                        function: String::from("pop_rules_by_hashes"),
                        line: String::from("popped_rules.len() == 0"),
                        msg: String::from("Could not find any hash in list.")
                    })
                }
                else if missed_hashes.len() == 0 {
                    Ok(EFOk{ value: popped_rules, msg: String::from("Removed all rules.") })
                }
                else {
                    let mut missed_hashes_str: String = String::from("Removed some rules. Missed hashes were ");
                    for missed_hash in missed_hashes {
                        missed_hashes_str.push_str(missed_hash.as_str());
                        missed_hashes_str.push_str(", ");
                    }
                    missed_hashes_str.push('.');

                    Ok(EFOk{ value: popped_rules, msg: missed_hashes_str })
                }
            }
        }
    }

    fn pop_rules_by_property(
        &mut self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>
    {
        match property_target {
            EFRulePropertyTarget::All => {
                // Clear allow rules
                self.allow_rules

                // Clear rules
                let temp_owner = self.rules;
                self.rules = Vec::new();
            }
        }
    }
}
