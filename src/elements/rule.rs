use std::hash::Hash;
use std::collections::HashMap;
use crate::utils::result::*;
use crate::utils::general::get_hash;
use crate::utils::hashmap::get_vector_of_keys_from_generic_hashmap;
use crate::elements::uri::EFURIString;
use crate::elements::tracker::EFItemTracker;

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
    AllForIdentity(EFURIString),
}

#[derive(Debug)]
pub enum EFRuleTarget<P: EFPrivilege> {
    Hash(EFRuleHashTarget),
    Id(EFRuleIdTarget),
    All(EFRulePropertyTarget<P>)
}

#[derive(Debug, Clone)]
pub struct EFRule<P: EFPrivilege> {
    identity: EFURIString,
    effect: EFRuleEffect,
    privilege: P
}

impl<P: EFPrivilege> EFRule<P> {
    pub fn to_hash(&self, rule_hash: &String) -> EFResult<String> {
        get_hash(
            vec![
                &self.identity.0, 
                &self.effect.as_str().to_string(), 
                &self.privilege.as_str().to_string()
            ], 
            rule_hash
        )
    }

    pub fn to_string_desc(&self) -> String {
        format!(
            "{} {} for {}", 
            self.identity.0.as_str(),
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
    GetIdentities,
    GetPrivileges,
    GetRules(EFRuleTarget<P>),
    AddRules(EFRuleIdTarget),
    PopRules(EFRuleTarget<P>),
}

#[derive(Debug)]
pub enum EFRuleTrackerResponse<P: EFPrivilege> {
    Count(usize),
    Hashes(EFRuleHashTarget),
    Identities(Vec<EFURIString>),
    Privileges(Vec<P>),
    Rules(Vec<EFRule<P>>),
    Error(EFError)
}

pub trait EFRuleTracker {
    type PrivilegeType: EFPrivilege;

    fn new() -> Self;

    // Checking the contents of the rule tracker
    fn get_rule_count(&self) -> EFResult<usize>;
    fn get_hashes(&self) -> EFResult<EFRuleHashTarget>;
    fn get_identities(&self) -> EFResult<Vec<EFURIString>>;
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

    // Request handler
    fn handle_rule_tracker_request(
        &self,
        request: EFRuleTrackerRequest<Self::PrivilegeType>
    ) -> EFResult<EFRuleTrackerResponse<Self::PrivilegeType>>;
}

#[derive(Debug)]
pub enum EFBasicRuleTrackerTryPopResult {
    OutOfBounds,
    Removed,
    RemovedAndEmpty
}

#[derive(Debug)]
pub struct EFBasicRuleHashEntry {
    rules_index: usize,
    allow_rules_index: Option<usize>,
    deny_rules_index: Option<usize>,
    identity_map_index: usize,
    privilege_map_index: usize
}

#[derive(Debug)]
pub struct EFBasicRuleTracker<P: EFPrivilege> {
    rules: EFItemTracker<EFRule<P>>,
    rule_hashes: HashMap<String, EFBasicRuleHashEntry>,
    allow_rules: EFItemTracker<usize>,
    deny_rules: EFItemTracker<usize>,
    identity_map: HashMap<EFURIString, EFItemTracker<usize>>,
    privilege_map: HashMap<P, EFItemTracker<usize>>
}

impl<P: EFPrivilege> EFBasicRuleTracker<P> {
    fn add_to_tracker(&mut self, rule_hash: String, rule_combo: &EFRule<P>) {
        // Insert to rules
        let rules_index: usize = self.rules.push_item(rule_combo.clone());

        // Insert to effect
        let allow_rules_index: Option<usize>;
        let deny_rules_index: Option<usize>;
        if let EFRuleEffect::Allow = rule_combo.effect {
            allow_rules_index = Some(self.allow_rules.push_item(rules_index));
            deny_rules_index = None;
        }
        else {
            allow_rules_index = None;
            deny_rules_index = Some(self.deny_rules.push_item(rules_index));
        }

        // Insert to identity map
        let identity_map_index: usize = match self.identity_map.get_mut(&rule_combo.identity) {
            Some(idenity_rules) => idenity_rules.push_item(rules_index),
            None => {
                let mut new_idenity_rules: EFItemTracker<usize> = EFItemTracker::new();
                let imi: usize = new_idenity_rules.push_item(rules_index);
                self.identity_map.insert(rule_combo.identity.clone(), new_idenity_rules);
                imi
            }
        };

        // Insert to privilege map
        let privilege_map_index: usize = match self.privilege_map.get_mut(&rule_combo.privilege) {
            Some(privilege_rules) => privilege_rules.push_item(rules_index),
            None => {
                let mut new_privilege_rules: EFItemTracker<usize> = EFItemTracker::new();
                let pmi: usize = new_privilege_rules.push_item(rules_index);
                self.privilege_map.insert(rule_combo.privilege.clone(), new_privilege_rules);
                pmi
            }
        };

        // Insert to hashes
        self.rule_hashes.insert(
            rule_hash,
            EFBasicRuleHashEntry { 
                rules_index, 
                allow_rules_index, 
                deny_rules_index, 
                identity_map_index, 
                privilege_map_index 
            }
        );
    }

    fn pop_from_tracker(&mut self, rule_hash: &String) -> EFResult<EFRule<P>> {
        // Try to get hash entry while removing
        let rule_entry: EFBasicRuleHashEntry = match self.rule_hashes.remove(rule_hash) {
            Some(r_e) => r_e,
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.rule_hashes.remove(rule_hash)"), 
                    msg: format!("Could not find hash {}.", rule_hash.as_str())
                });
            }
        };

        // Pop from rules
        let rule: EFRule<P> = match self.rules.pop_item(rule_entry.rules_index) {
            Some(r) => r,
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.rules.pop_item(rule_entry.rules_index)"), 
                    msg: format!("Could not find rule for {}.", rule_hash.as_str())
                });
            }
        };

        // Pop from allow rules if needed
        if let Some(allow_rules_index) = rule_entry.allow_rules_index {
            self.allow_rules.pop_item(allow_rules_index);
        }

        // Pop from deny rules if needed
        if let Some(deny_rules_index) = rule_entry.deny_rules_index {
            self.deny_rules.pop_item(deny_rules_index);
        }

        // Pop from identities
        let identity_rules_length: usize = match self.identity_map.get_mut(&rule.identity) {
            Some(identity_rules) => {
                identity_rules.pop_item(rule_entry.identity_map_index);
                identity_rules.get_length()
            },
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.identity_map.get_mut(&rule.identity)"), 
                    msg: format!("ID {} does not have any rules.", rule.identity.0.as_str())
                });
            }
        };
        if identity_rules_length == 0 {
            self.identity_map.remove(&rule.identity);
        }

        // Pop from privileges
        let privileges_rules_length: usize = match self.privilege_map.get_mut(&rule.privilege) {
            Some(privilege_rules) => {
                privilege_rules.pop_item(rule_entry.privilege_map_index);
                privilege_rules.get_length()
            },
            None => {
                return Err(EFError{
                    function: String::from("pop_from_tracker"), 
                    line: String::from("self.privilege_map.get_mut(&rule.privilege)"), 
                    msg: format!("Privilege {} does not have any rules.", rule.privilege.as_str())
                });
            }
        };
        if privileges_rules_length == 0 {
            self.privilege_map.remove(&rule.privilege);
        }

        // Return the rule
        Ok(EFOk{
            value: rule, 
            msg: format!("Popped rule {} from tracker.", rule_hash.as_str())
        })
    }

    fn compact_tracker(&mut self) -> EFResult<EFSuccess> {
        // Compact rules
        let rules_translation_map: HashMap<usize, usize> = self.rules.compact_items();

        // Compact allow rules
        let allow_rules_translation_map: HashMap<usize, usize> = self.allow_rules.compact_items();

        // Compact deny rules
        let deny_rules_translation_map: HashMap<usize, usize> = self.deny_rules.compact_items();

        // Compact identity map
        let mut identity_translation_set: HashMap<&EFURIString, HashMap<usize, usize>> = HashMap::new();
        for (identity, identity_rules) in self.identity_map.iter_mut() {
            identity_translation_set.insert(identity, identity_rules.compact_items());
        }

        // Compact privilege map
        let mut privilege_translation_set: HashMap<&P, HashMap<usize, usize>> = HashMap::new();
        for (privilege, privilege_rules) in self.privilege_map.iter_mut() {
            privilege_translation_set.insert(privilege, privilege_rules.compact_items());
        }

        // Update rule hashes
        for (_, rule_entry) in self.rule_hashes.iter_mut() {
            // Update rules index
            rule_entry.rules_index = match rules_translation_map.get(&rule_entry.rules_index) {
                Some(new_rules_index) => new_rules_index.clone(),
                None => rule_entry.rules_index
            };

            // Update allow rules index if needed
            if let Some(old_allow_rules_index) = rule_entry.allow_rules_index {
                rule_entry.allow_rules_index = match allow_rules_translation_map.get(&old_allow_rules_index) {
                    Some(new_allow_rules_index) => Some(new_allow_rules_index.clone()),
                    None => rule_entry.allow_rules_index
                };
            }

            // Update deny rules index if needed
            if let Some(old_deny_rules_index) = rule_entry.deny_rules_index {
                rule_entry.deny_rules_index = match deny_rules_translation_map.get(&old_deny_rules_index) {
                    Some(new_deny_rules_index) => Some(new_deny_rules_index.clone()),
                    None => rule_entry.deny_rules_index
                };
            }

            // Get current rule
            let rule: &EFRule<P> = match self.rules.get_item(rule_entry.rules_index) {
                Some(r) => r,
                None => {
                    return Err(EFError{
                        function: String::from("compact_tracker"), 
                        line: String::from("self.rules.get_item(rule_entry.rules_index)"), 
                        msg: format!(
                            "Cannot get rule at new rules index {} to continue compation operation.", 
                            rule_entry.rules_index
                        )
                    });
                }
            };

            // Update identity map index
            rule_entry.identity_map_index = match identity_translation_set.get(&rule.identity) {
                Some(identity_translation_map) => match identity_translation_map.get(&rule_entry.identity_map_index) {
                    Some(new_identity_map_index) => new_identity_map_index.clone(),
                    None => rule_entry.identity_map_index
                },
                None => {
                    return Err(EFError{
                        function: String::from("compact_tracker"), 
                        line: String::from("identity_translation_set.get(&rule.identity)"), 
                        msg: format!(
                            "Cannot find identity {} in identity translation set to continue compation operation.", 
                            rule.identity.0.as_str()
                        )
                    });
                }
            };

            // Update privilege map index
            rule_entry.privilege_map_index = match privilege_translation_set.get(&rule.privilege) {
                Some(privilege_translation_map) => match privilege_translation_map.get(&rule_entry.privilege_map_index) {
                    Some(new_privilege_map_index) => new_privilege_map_index.clone(),
                    None => rule_entry.privilege_map_index
                },
                None => {
                    return Err(EFError{
                        function: String::from("compact_tracker"), 
                        line: String::from("privilege_translation_set.get(&rule.privilege)"), 
                        msg: format!(
                            "Cannot find privilege {} in privilege translation set to continue compation operation.", 
                            rule.privilege.as_str()
                        )
                    });
                }
            };
        }

        Ok(EFOk { value: EFSuccess, msg: String::from("Compacted tracker.") })
    }
}

impl<P: EFPrivilege> EFRuleTracker for EFBasicRuleTracker<P> {
    type PrivilegeType = P;

    fn new() -> Self {
        EFBasicRuleTracker {
            rules: EFItemTracker::new(), 
            rule_hashes: HashMap::new(), 
            allow_rules: EFItemTracker::new(), 
            deny_rules: EFItemTracker::new(), 
            identity_map: HashMap::new(), 
            privilege_map: HashMap::new()
        }
    }

    fn get_rule_count(&self) -> EFResult<usize> {
        Ok(EFOk{
            value: self.rules.get_length(),
            msg: String::from("Got length of rules.")
        })
    }

    fn get_hashes(&self) -> EFResult<EFRuleHashTarget> {
        match get_vector_of_keys_from_generic_hashmap(&self.rule_hashes) {
            Ok(hashes) => Ok(EFOk{
                value: EFRuleHashTarget::MultipleHash(hashes.value),
                msg: String::from("Got vector of hashes.")
            }),
            Err(e) => Err(e)
        }
    }

    fn get_identities(&self) -> EFResult<Vec<EFURIString>> {
        match get_vector_of_keys_from_generic_hashmap(&self.identity_map) {
            Ok(identities) => Ok(EFOk{
                value: identities.value, 
                msg: String::from("Got vector of identities.")
            }),
            Err(e) => Err(e)
        }
    }

    fn get_privileges(&self) -> EFResult<Vec<Self::PrivilegeType>> {
        match get_vector_of_keys_from_generic_hashmap(&self.privilege_map) {
            Ok(privileges) => Ok(EFOk{
                value: privileges.value,
                msg: String::from("Got vector of identities.")
            }),
            Err(e) => Err(e)
        }
    }

    fn get_rules_by_hashes(&self, hash_target: &EFRuleHashTarget) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>> {
        match hash_target {
            EFRuleHashTarget::SingleHash(hash) => match self.rule_hashes.get(hash) {
                Some(hash_entry) => match self.rules.get_item(hash_entry.rules_index) {
                    Some(rule) => Ok(EFOk{
                        value: vec![rule], 
                        msg: format!("Found rule for {}.", hash.as_str())
                    }),
                    None => Err(EFError{
                        function: String::from("get_rules_by_hashes"), 
                        line: String::from("self.rules.get_item(hash_entry.rules_index)"), 
                        msg: format!("Could not get rule for {}.", hash.as_str())
                    })
                },
                None => Err(EFError{
                    function: String::from("get_rules_by_hashes"), 
                    line: String::from("self.rule_hashes.get(hash)"), 
                    msg: format!("Could not find hash {}.", hash.as_str())
                })
            },
            EFRuleHashTarget::MultipleHash(hashes) => {
                let mut found_rules: Vec<&EFRule<Self::PrivilegeType>> = Vec::new();
                let mut unknown_hashes: Vec<&String> = Vec::new();
                let mut missing_rules: Vec<&String> = Vec::new();

                // Loop through hashes to search for rules
                for hash in hashes {
                    match self.rule_hashes.get(hash) {
                        Some(hash_entry) => match self.rules.get_item(hash_entry.rules_index) {
                            Some(rule) => { found_rules.push(rule); },
                            None => { missing_rules.push(hash); }
                        },
                        None => { unknown_hashes.push(hash); }
                    }
                }

                // Early exit if no rules were found
                if found_rules.is_empty() {
                    return Err(EFError{
                        function: String::from("get_rules_by_hashes"), 
                        line: String::from("found_rules.is_empty()"), 
                        msg: String::from("Could not find any requested rule.")
                    });
                }

                // Build return result
                let unknown_hashes_str: String = match unknown_hashes.is_empty() {
                    true => String::from("None"),
                    false => {
                        let mut temp_str: String = String::new();
                        for unknown_hash in unknown_hashes {
                            temp_str.push_str(unknown_hash.as_str());
                            temp_str.push(',');
                        }
                        temp_str
                    }
                };
                let missing_rules_str: String = match missing_rules.is_empty() {
                    true => String::from("None"),
                    false => {
                        let mut temp_str: String = String::new();
                        for missing_rule in missing_rules {
                            temp_str.push_str(missing_rule.as_str());
                            temp_str.push(',');
                        }
                        temp_str
                    }
                };
                let found_rules_length: usize = found_rules.len();

                Ok(EFOk{
                    value: found_rules, 
                    msg: format!(
                        "Found {} rules. The following hashes could not be found: {}. 
                        The following hashes were found but had no rules {}.",
                        found_rules_length,
                        unknown_hashes_str.as_str(),
                        missing_rules_str.as_str()
                    )
                })
            }
        }
    }

    fn get_rules_by_property(
        &self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>>
    {
        let get_property_pipeline = | property_indexes: Vec<&usize> | -> Vec<&EFRule<Self::PrivilegeType>> {
            property_indexes.into_iter()
            .map(|item_index| self.rules.get_item(item_index.clone()))
            .filter(|item_option| item_option.is_some())
            .map(|item_some| item_some.unwrap())
            .collect()
        };

        match property_target {
            EFRulePropertyTarget::All => Ok(EFOk{
                value: self.rules.get_all_items(),
                msg: String::from("Got all rules.")
            }),
            EFRulePropertyTarget::AllForEffect(effect) => match effect {
                EFRuleEffect::Allow => Ok(EFOk{
                    value: get_property_pipeline(self.allow_rules.get_all_items()), 
                    msg: String::from("Got all allow rules.")
                }),
                EFRuleEffect::Deny => Ok(EFOk{
                    value: get_property_pipeline(self.deny_rules.get_all_items()), 
                    msg: String::from("Got all deny rules.")
                })
            },
            EFRulePropertyTarget::AllForPrivilege(privilege) => match self.privilege_map.get(privilege) {
                Some(privilege_rules) => Ok(EFOk{
                    value: get_property_pipeline(privilege_rules.get_all_items()), 
                    msg: format!("Got rules for privilege {}.", privilege.as_str())
                }),
                None => Err(EFError{
                    function: String::from("get_rules_by_property"),
                    line: String::from("self.privilege_map.get(privilege)"),
                    msg: format!("Could not get rules for privilege {}.", privilege.as_str())
                })
            },
            EFRulePropertyTarget::AllForIdentity(identity) => match self.identity_map.get(identity) {
                Some(identity_rules) => Ok(EFOk{
                    value: get_property_pipeline(identity_rules.get_all_items()), 
                    msg: format!("Got rules for identity {}.", identity.0.as_str())
                }),
                None => Err(EFError{
                    function: String::from("get_rules_by_property"),
                    line: String::from("self.identity_map.get(identity)"),
                    msg: format!("Could not get rules for identity {}.", identity.0.as_str())
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
                    identity: single_id.clone(),
                    effect: anon_rule.effect.clone(),
                    privilege: anon_rule.privilege.clone()
                });
            }
        }
        else if let EFRuleIdTarget::MultipleId(multiple_ids) = id_target {
            for single_id in multiple_ids {
                for anon_rule in anon_rules {
                    rule_combos.push(EFRule{
                        identity: single_id.clone(),
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

            if let None = self.rule_hashes.get(&rule_combo_hash) {
                output_target.push(rule_combo_hash.clone());
                self.add_to_tracker(rule_combo_hash, &rule_combo);
            }
            else {
                existing_rules.push_str(rule_combo.to_string_desc().as_str());
                existing_rules.push(',');
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
            EFRuleHashTarget::SingleHash(hash) => match self.pop_from_tracker(hash) {
                Ok(rule) => Ok(EFOk{
                    value: vec![rule.value], 
                    msg: format!("Popped rule for {}.", hash.as_str())
                }),
                Err(e) => Err(e)
            },
            EFRuleHashTarget::MultipleHash(hashes) => {
                let mut popped_rules: Vec<EFRule<Self::PrivilegeType>> = Vec::new();
                let mut missed_hashes: Vec<&String> = Vec::new();

                for hash in hashes {
                    match self.pop_from_tracker(hash) {
                        Ok(rule) => { popped_rules.push(rule.value); },
                        Err(_) => { missed_hashes.push(hash); }
                    }
                }

                // Early exit if no rules were popped
                if popped_rules.is_empty() {
                    return Err(EFError{
                        function: String::from("pop_rules_by_hashes"), 
                        line: String::from("popped_rules.is_empty()"), 
                        msg: String::from("Could not pop any requested rule.")
                    });
                }

                // Build return result
                let missed_hashes_str: String = match missed_hashes.is_empty() {
                    true => String::from("None"),
                    false => {
                        let mut temp_str: String = String::new();
                        for missed_hash in missed_hashes {
                            temp_str.push_str(missed_hash.as_str());
                            temp_str.push(',');
                        }
                        temp_str
                    }
                };
                let popped_rules_length: usize = popped_rules.len();

                Ok(EFOk{
                    value: popped_rules, 
                    msg: format!{
                        "Popped {} rule. The following hashes could not be popped: {}.",
                        popped_rules_length,
                        missed_hashes_str.as_str()
                    }
                })
            }
        }
    }

    fn pop_rules_by_property(
        &mut self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>
    {
        let mut pop_property_pipeline = | property_indexes: Vec<usize> | -> Vec<EFRule<Self::PrivilegeType>> {
            property_indexes.into_iter()
            .map(|item_index| self.rules.pop_item(item_index))
            .filter(|item_option| item_option.is_some())
            .map(|item_some| item_some.unwrap())
            .collect()
        };

        match property_target {
            EFRulePropertyTarget::All => Ok(EFOk{
                value: self.rules.pop_all_items(), 
                msg: String::from("Popped all rules.")
            }),
            EFRulePropertyTarget::AllForEffect(effect) => match effect {
                EFRuleEffect::Allow => Ok(EFOk{
                    value: pop_property_pipeline(self.allow_rules.pop_all_items()), 
                    msg: String::from("Popped all allow rules.")
                }),
                EFRuleEffect::Deny => Ok(EFOk{
                    value: pop_property_pipeline(self.deny_rules.pop_all_items()), 
                    msg: String::from("Popped all deny rules.")
                })
            },
            EFRulePropertyTarget::AllForPrivilege(privilege) => match self.privilege_map.get_mut(privilege) {
                Some(privilege_rules) => Ok(EFOk{
                    value: pop_property_pipeline(privilege_rules.pop_all_items()), 
                    msg: format!("Popped rules for privilege {}.", privilege.as_str())
                }),
                None => Err(EFError{
                    function: String::from("pop_rules_by_property"),
                    line: String::from("self.privilege_map.get(privilege)"),
                    msg: format!("Could not get rules for privilege {}.", privilege.as_str())
                })
            },
            EFRulePropertyTarget::AllForIdentity(identity) => match self.identity_map.get_mut(identity) {
                Some(identity_rules) => Ok(EFOk{
                    value: pop_property_pipeline(identity_rules.pop_all_items()), 
                    msg: format!("Popped rules for identity {}.", identity.0.as_str())
                }),
                None => Err(EFError{
                    function: String::from("pop_rules_by_property"),
                    line: String::from("self.identity_map.get(identity)"),
                    msg: format!("Could not get rules for identity {}.", identity.0.as_str())
                })
            }
        }
    }

    fn handle_rule_tracker_request(
        &self,
        request: EFRuleTrackerRequest<Self::PrivilegeType>
    ) -> EFRuleTrackerResponse<Self::PrivilegeType>
    {
        match request {
            EFRuleTrackerRequest::GetRuleCount => match self.get_rule_count() {
                Ok(rule_count) => EFRuleTrackerResponse::Count(rule_count.value),
                Err(e) => EFRuleTrackerResponse::Error(e)
            },
            EFRuleTrackerRequest::GetHashes => match self.get_hashes() {
                Ok(hashes) => EFRuleTrackerResponse::Hashes(hashes.value),
                Err(e) => EFRuleTrackerResponse::Error(e)
            },
            EFRuleTrackerRequest::GetIdentities => match self.get_identities() {
                Ok(identities) => EFRuleTrackerResponse::Identities(identities.value),
                Err(e) => EFRuleTrackerResponse::Error(e)
            },
            EFRuleTrackerRequest::GetPrivileges => match self.get_privileges() {
                Ok(privileges) => EFRuleTrackerResponse::Privileges(privileges.value),
                Err(e) => EFRuleTrackerResponse::Error(e)
            },
            EFRuleTrackerRequest::GetRules(target) =>,
            EFRuleTrackerRequest::AddRules(id_target) =>,
            EFRuleTrackerRequest::PopRules(target) =>,
        }
    }
}
