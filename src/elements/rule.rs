use std::hash::Hash;
use std::collections::HashMap;
use crate::utils::result::{EFValueResult, EFReturnEvent, EFResult, EFReturn};
use crate::utils::general::get_hash;
use crate::utils::hashmap::get_keys_vec;
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
    Single(String),
    Multiple(Vec<String>)
}

#[derive(Debug)]
pub enum EFRuleIdentityTarget {
    Single(EFURIString),
    Multiple(Vec<EFURIString>),
}

#[derive(Debug)]
pub enum EFRulePropertyTarget<P: EFPrivilege> {
    All,
    Effect(EFRuleEffect),
    Privilege(P),
    Identity(EFURIString),
}

#[derive(Debug)]
pub enum EFRuleTarget<P: EFPrivilege> {
    Hash(EFRuleHashTarget),
    Identity(EFRuleIdentityTarget),
    Property(EFRulePropertyTarget<P>)
}

#[derive(Debug, Clone)]
pub struct EFRule<P: EFPrivilege> {
    identity: EFURIString,
    effect: EFRuleEffect,
    privilege: P
}

impl<P: EFPrivilege> EFRule<P> {
    pub fn to_hash(&self, rule_hash: &String) -> EFValueResult<String> {
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
    AddRules(EFRuleIdentityTarget, Vec<EFAnonRule<P>>, String),
    PopRules(EFRuleTarget<P>)
}

#[derive(Debug)]
pub enum EFRuleTrackerResponse<P: EFPrivilege> {
    RuleCount(usize),
    Hashes(EFRuleHashTarget),
    Identities(Vec<EFURIString>),
    Privileges(Vec<P>),
    RetrievedRules(EFReturn<Vec<EFRule<P>>>),
    AddedRules(EFReturn<EFRuleHashTarget>),
    Error(EFReturnEvent)
}

pub trait EFRuleTracker { 
    type PrivilegeType: EFPrivilege;

    fn new() -> Self;

    // Checking the contents of the rule tracker
    fn get_rule_count(&self) -> usize;
    fn get_hashes(&self) -> EFRuleHashTarget;
    fn get_identities(&self) -> Vec<EFURIString>;
    fn get_privileges(&self) -> Vec<Self::PrivilegeType>;

    // Getting rules
    fn get_rules_by_hashes(&self, hash_target: &EFRuleHashTarget) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>>;
    fn get_rules_by_property(
        &self,
        property_target: &EFRulePropertyTarget<Self::PrivilegeType>
    ) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>>;

    // Adding rules
    fn add_rules(
        &mut self,
        id_target: &EFRuleIdentityTarget,
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

    // Request handlers
    fn handle_rule_request(
        &mut self,
        request: EFRuleTrackerRequest<Self::PrivilegeType>
    ) -> EFRuleTrackerResponse<Self::PrivilegeType>;
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
                return Err(EFReturnEvent::new_with_func_info_log(
                    "pop_from_tracker", 
                    format!("Could not find hash {}.", rule_hash.as_str()).as_str()
                ));
            }
        };

        // Pop from rules
        let rule: EFRule<P> = match self.rules.pop_item(rule_entry.rules_index) {
            Some(r) => r,
            None => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "pop_from_tracker", 
                    format!("Could not find rule for {}.", rule_hash.as_str()).as_str()
                ));
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
                return Err(EFReturnEvent::new_with_func_info_log(
                    "pop_from_tracker", 
                    format!("ID {} does not have any rules.", rule.identity.0.as_str()).as_str()
                ));
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
                return Err(EFReturnEvent::new_with_func_info_log(
                    "pop_from_tracker", 
                    format!("Privilege {} does not have any rules.", rule.privilege.as_str()).as_str()
                ));
            }
        };
        if privileges_rules_length == 0 {
            self.privilege_map.remove(&rule.privilege);
        }

        // Return the rule
        Ok(EFReturn{
            value: rule,
            event: EFReturnEvent::new_with_func_info_log(
                "pop_from_tracker", 
                format!("Popped rule {} from tracker.", rule_hash.as_str()).as_str()
            )
        })
    }

    fn compact_tracker(&mut self) -> EFValueResult<()> {
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
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "compact_tracker", 
                        format!(
                            "Cannot get rule at new rules index {} to continue compation operation.", 
                            rule_entry.rules_index
                        ).as_str()
                    ));
                }
            };

            // Update identity map index
            rule_entry.identity_map_index = match identity_translation_set.get(&rule.identity) {
                Some(identity_translation_map) => match identity_translation_map.get(&rule_entry.identity_map_index) {
                    Some(new_identity_map_index) => new_identity_map_index.clone(),
                    None => rule_entry.identity_map_index
                },
                None => {
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "compact_tracker", 
                        format!(
                            "Cannot find identity {} in identity translation set to continue compation operation.", 
                            rule.identity.0.as_str()
                        ).as_str()
                    ));
                }
            };

            // Update privilege map index
            rule_entry.privilege_map_index = match privilege_translation_set.get(&rule.privilege) {
                Some(privilege_translation_map) => match privilege_translation_map.get(&rule_entry.privilege_map_index) {
                    Some(new_privilege_map_index) => new_privilege_map_index.clone(),
                    None => rule_entry.privilege_map_index
                },
                None => {
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "compact_tracker", 
                        format!(
                            "Cannot find privilege {} in privilege translation set to continue compation operation.", 
                            rule.privilege.as_str()
                        ).as_str()
                    ));
                }
            };
        }

        Ok(())
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

    fn get_rule_count(&self) -> usize {
        self.rules.get_length()
    }

    fn get_hashes(&self) -> EFRuleHashTarget {
        EFRuleHashTarget::Multiple(get_keys_vec(&self.rule_hashes))
    }

    fn get_identities(&self) -> Vec<EFURIString> {
        get_keys_vec(&self.identity_map)
    }

    fn get_privileges(&self) -> Vec<Self::PrivilegeType> {
        get_keys_vec(&self.privilege_map)
    }

    fn get_rules_by_hashes(&self, hash_target: &EFRuleHashTarget) -> EFResult<Vec<&EFRule<Self::PrivilegeType>>> {
        // Setup return items
        let mut found_rules: Vec<&EFRule<Self::PrivilegeType>> = Vec::new();
        let mut found_rules_event: EFReturnEvent = EFReturnEvent::new();

        // Setup closure
        let get_hash_closure = | temp_hash: &String | {
            match self.rule_hashes.get(temp_hash) {
                Some(hash_entry) => match self.rules.get_item(hash_entry.rules_index) {
                    Some(rule) => {
                        found_rules.push(rule);
                        found_rules_event.add_func_info_log(
                            "get_rules_by_hashes", 
                            format!("Got rule for {}.", temp_hash.as_str()).as_str()
                        );
                    },
                    None => {
                        found_rules_event.add_func_info_log(
                            "get_rules_by_hashes", 
                            format!("Could not get rule for {}.", temp_hash.as_str()).as_str()
                        );
                    }
                },
                None => {
                    found_rules_event.add_func_info_log(
                        "get_rules_by_hashes", 
                        format!("Could not find rule {}.", temp_hash.as_str()).as_str()
                    );
                }
            }
        };

        // Do match logic
        match hash_target {
            EFRuleHashTarget::Single(hash) => { get_hash_closure(hash); },
            EFRuleHashTarget::Multiple(hashes) => {
                for hash in hashes {
                    get_hash_closure(hash);
                }
            }
        }

        // Check if any rules were found
        if found_rules.is_empty() {
            Err(found_rules_event)
        }
        else {
            Ok(EFReturn{
                value: found_rules,
                event: found_rules_event
            })
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
            EFRulePropertyTarget::All => Ok(EFReturn{
                value: self.rules.get_all_items(), 
                event: EFReturnEvent::new_with_func_info_log(
                    "get_rules_by_property", 
                    "Got all rules."
                )
            }),
            EFRulePropertyTarget::Effect(effect) => match effect {
                EFRuleEffect::Allow => Ok(EFReturn{
                    value: get_property_pipeline(self.allow_rules.get_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "get_rules_by_property", 
                        "Got all allow rules."
                    )
                }),
                EFRuleEffect::Deny => Ok(EFReturn{
                    value: get_property_pipeline(self.deny_rules.get_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "get_rules_by_property", 
                        "Got all deny rules."
                    )
                })
            },
            EFRulePropertyTarget::Privilege(privilege) => match self.privilege_map.get(privilege) {
                Some(privilege_rules) => Ok(EFReturn{
                    value: get_property_pipeline(privilege_rules.get_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "get_rules_by_property", 
                        format!("Got rules for privilege {}.", privilege.as_str()).as_str()
                    )
                }),
                None => Err(EFReturnEvent::new_with_func_info_log(
                    "get_rules_by_property", 
                    format!("Could not get rules for privilege {}.", privilege.as_str()).as_str()
                ))
            },
            EFRulePropertyTarget::Identity(identity) => match self.identity_map.get(identity) {
                Some(identity_rules) => Ok(EFReturn{
                    value: get_property_pipeline(identity_rules.get_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "get_rules_by_property", 
                        format!("Got rules for identity {}.", identity.0.as_str()).as_str()
                    )
                }),
                None => Err(EFReturnEvent::new_with_func_info_log(
                    "get_rules_by_property", 
                    format!("Could not get rules for identity {}.", identity.0.as_str()).as_str()
                ))
            }
        }
    }

    fn add_rules(
        &mut self,
        id_target: &EFRuleIdentityTarget,
        anon_rules: &Vec<EFAnonRule<Self::PrivilegeType>>,
        rule_hash: &String
    ) -> EFResult<EFRuleHashTarget>
    {
        // Create rule combinations
        let mut rule_combos: Vec<EFRule<Self::PrivilegeType>> = Vec::new();
        match id_target {
            EFRuleIdentityTarget::Single(identity) => {
                for anon_rule in anon_rules {
                    rule_combos.push(EFRule{
                        identity: identity.clone(),
                        effect: anon_rule.effect.clone(),
                        privilege: anon_rule.privilege.clone()
                    });
                }
            },
            EFRuleIdentityTarget::Multiple(identities) => {
                for identity in identities {
                    for anon_rule in anon_rules {
                        rule_combos.push(EFRule{
                            identity: identity.clone(),
                            effect: anon_rule.effect.clone(),
                            privilege: anon_rule.privilege.clone()
                        });
                    }
                }
            }
        }

        // Try adding to tracker
        let mut output_hashes: Vec<String> = Vec::new();
        let mut try_add_event: EFReturnEvent = EFReturnEvent::new();

        for rule_combo in rule_combos {
            // Create the hash
            let rule_combo_hash: String = match rule_combo.to_hash(rule_hash) {
                Ok(h) => h,
                Err(e) => {
                    // Return early if the problem is the hasher setting
                    if e.get_default_delimited_logs().contains("incorrect value") {
                        return Err(e);
                    }

                    // Add any other problems to the event logs
                    try_add_event.transfer_event(e);
                    try_add_event.add_func_info_log(
                        "add_rules", 
                        format!(
                            "Rule combo '{}' could not be turned into a hash.",
                            rule_combo.to_string_desc().as_str()
                        ).as_str()
                    );

                    // Early loop end
                    continue;
                }
            };

            if let None = self.rule_hashes.get(&rule_combo_hash) {
                try_add_event.add_func_info_log(
                    "add_rules", 
                    format!(
                        "Rule combo '{}' was added.",
                        rule_combo.to_string_desc().as_str()
                    ).as_str()
                );
                output_hashes.push(rule_combo_hash.clone());
                self.add_to_tracker(rule_combo_hash, &rule_combo);
            }
            else {
                try_add_event.add_func_info_log(
                    "add_rules", 
                    format!(
                        "Rule combo '{}' could not be added as it already exists.",
                        rule_combo.to_string_desc().as_str()
                    ).as_str()
                );
            }
        }

        // Return information
        Ok(EFReturn{
            value: EFRuleHashTarget::Multiple(output_hashes),
            event: try_add_event
        })
    }

    fn pop_rules_by_hashes(
        &mut self,
        hash_target: &EFRuleHashTarget
    ) -> EFResult<Vec<EFRule<Self::PrivilegeType>>>
    {
        // Setup return items
        let mut popped_rules: Vec<EFRule<Self::PrivilegeType>> = Vec::new();
        let mut popped_rules_event: EFReturnEvent = EFReturnEvent::new();

        // Setup closure
        let pop_hash_closure = | temp_hash: &String | {
            match self.pop_from_tracker(temp_hash) {
                Ok(rule_return) => {
                    popped_rules.push(
                        popped_rules_event.strip_event_from_return(rule_return)
                    );
                },
                Err(e) => { popped_rules_event.transfer_event(e); }
            }
        };

        match hash_target {
            EFRuleHashTarget::Single(hash) => { pop_hash_closure(hash); },
            EFRuleHashTarget::Multiple(hashes) => {
                for hash in hashes {
                    pop_hash_closure(hash);
                }
            }
        }

        // Check if any rules were popped
        if popped_rules.is_empty() {
            Err(popped_rules_event)
        }
        else {
            Ok(EFReturn{
                value: popped_rules,
                event: popped_rules_event
            })
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
            EFRulePropertyTarget::All => Ok(EFReturn{
                value: self.rules.pop_all_items(), 
                event: EFReturnEvent::new_with_func_info_log(
                    "pop_rules_by_property", 
                    "Popped all rules."
                )
            }),
            EFRulePropertyTarget::Effect(effect) => match effect {
                EFRuleEffect::Allow => Ok(EFReturn{
                    value: pop_property_pipeline(self.allow_rules.pop_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "pop_rules_by_property", 
                        "Popped all allow rules."
                    )
                }),
                EFRuleEffect::Deny => Ok(EFReturn{
                    value: pop_property_pipeline(self.deny_rules.pop_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "pop_rules_by_property", 
                        "Popped all deny rules."
                    )
                })
            },
            EFRulePropertyTarget::Privilege(privilege) => match self.privilege_map.get_mut(privilege) {
                Some(privilege_rules) => Ok(EFReturn{
                    value: pop_property_pipeline(privilege_rules.pop_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "pop_rules_by_property", 
                        format!("Popped rules for privilege {}.", privilege.as_str()).as_str()
                    )
                }),
                None => Err(EFReturnEvent::new_with_func_info_log(
                    "pop_rules_by_property", 
                    format!("Could not pop rules for privilege {}.", privilege.as_str()).as_str()
                ))
            },
            EFRulePropertyTarget::Identity(identity) => match self.identity_map.get_mut(identity) {
                Some(identity_rules) => Ok(EFReturn{
                    value: pop_property_pipeline(identity_rules.pop_all_items()), 
                    event: EFReturnEvent::new_with_func_info_log(
                        "pop_rules_by_property", 
                        format!("Popped rules for identity {}.", identity.0.as_str()).as_str()
                    )
                }),
                None => Err(EFReturnEvent::new_with_func_info_log(
                    "pop_rules_by_property", 
                    format!("Could not pop rules for identity {}.", identity.0.as_str()).as_str()
                ))
            }
        }
    }

    fn handle_rule_request(
        &mut self,
        request: EFRuleTrackerRequest<Self::PrivilegeType>
    ) -> EFRuleTrackerResponse<Self::PrivilegeType>
    {
        let create_retrieved_rules = | rules_result: EFReturn<Vec<&EFRule<Self::PrivilegeType>>> | -> EFRuleTrackerResponse<Self::PrivilegeType> {
            let (rules_value, rules_event) = rules_result.decompose();
            let compatible_rules: Vec<EFRule<Self::PrivilegeType>> = rules_value.into_iter().map(|r| r.clone()).collect();
            EFRuleTrackerResponse::RetrievedRules(
                EFReturn::compose(compatible_rules, rules_event)
            )
        };
        
        match request {
            EFRuleTrackerRequest::GetRuleCount => EFRuleTrackerResponse::RuleCount(self.get_rule_count()),
            EFRuleTrackerRequest::GetHashes => EFRuleTrackerResponse::Hashes(self.get_hashes()),
            EFRuleTrackerRequest::GetIdentities => EFRuleTrackerResponse::Identities(self.get_identities()),
            EFRuleTrackerRequest::GetPrivileges => EFRuleTrackerResponse::Privileges(self.get_privileges()),
            EFRuleTrackerRequest::GetRules(target) => {
                match target {
                    EFRuleTarget::Hash(hash_target) => match self.get_rules_by_hashes(&hash_target) {
                        Ok(gotten_rules) => create_retrieved_rules(gotten_rules),
                        Err(e) => EFRuleTrackerResponse::Error(e)
                    },
                    EFRuleTarget::Property(property_target) => match self.get_rules_by_property(&property_target) {
                        Ok(gotten_rules) => create_retrieved_rules(gotten_rules),
                        Err(e) => EFRuleTrackerResponse::Error(e)
                    },
                    _ => EFRuleTrackerResponse::Error(EFReturnEvent::new_with_func_info_log(
                        "handle_request", 
                        "Get rules target was not a hash target or property target."
                    ))
                }
            },
            EFRuleTrackerRequest::AddRules(id_target, anon_rules, rule_hash) => match self.add_rules(&id_target, &anon_rules, &rule_hash) {
                Ok(new_hashes) => EFRuleTrackerResponse::AddedRules(new_hashes),
                Err(e) => EFRuleTrackerResponse::Error(e)
            },
            EFRuleTrackerRequest::PopRules(target) => {
                match target {
                    EFRuleTarget::Hash(hash_target) => match self.pop_rules_by_hashes(&hash_target) {
                        Ok(popped_rules) => EFRuleTrackerResponse::RetrievedRules(popped_rules),
                        Err(e) => EFRuleTrackerResponse::Error(e)
                    },
                    EFRuleTarget::Property(property_target) => match self.pop_rules_by_property(&property_target) {
                        Ok(popped_rules) => EFRuleTrackerResponse::RetrievedRules(popped_rules),
                        Err(e) => EFRuleTrackerResponse::Error(e)
                    },
                    _ => EFRuleTrackerResponse::Error(EFReturnEvent::new_with_func_info_log(
                        "handle_request", 
                        "Pop rules target was not a hash target or property target."
                    ))
                }
            }
        }
    }
}
