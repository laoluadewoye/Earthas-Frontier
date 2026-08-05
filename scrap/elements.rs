pub mod entity {
    #[derive(Debug)]
    pub struct EFGID {
        local_id: usize,
        parent_id: usize,
        system_level: usize
    }

    #[derive(Debug)]
    pub struct EFGIDString(String);

    impl EFGID {
        pub fn new(local_id: usize, parent_id: usize, system_level: usize) -> EFGID {
            EFGID { local_id, parent_id, system_level }
        }

        fn get_local_id(&self) -> usize {
            self.local_id
        }

        fn get_parent_id(&self) -> usize {
            self.parent_id
        }

        fn get_system_level(&self) -> usize {
            self.system_level
        }

        fn combine_id(&self, is_system: bool) -> EFGIDString {
            match is_system {
                false => EFGIDString(format!("{0}-{1}-{2}", self.system_level, self.parent_id, self.local_id)),
                true => EFGIDString(format!("s-{0}-{1}-{2}", self.system_level, self.parent_id, self.local_id))
            }
        }
    }

    #[derive(Debug)]
    pub struct EFEntity<T> {
        pub gid: EFGID,
        pub name: String,
        pub owner: EFGIDString,
        pub object: T,
        pub object_type: String,
    }
}

mod identity {
    enum EFIdentityType {
        User,
        Role
    }

    struct EFIdentity {
        name: String,
        creation_date: String,
        modificaton_date: String,
        identity_type: EFIdentityType,
        password_hash: Option<String>
    }

    struct EFIdentityMap {
        user: String,
        roles: Vec<String>
    }
}

mod rules {
    #[derive(Debug)]
    enum EFRuleAction {
        Deny,
        Allow
    }

    #[derive(Debug)]
    enum EFSystemPrivilege {
        SeeSystem,
        SeeSystemMetadata,
        SeeSystemEntities,
        SeeSubsystems,
        SeeParentSystem,
        ModifySystem,
        CreateEntities,
        DeleteEntities,
        DeleteSystem
    }

    #[derive(Debug)]
    pub struct EFSystemRule {
        identities: Option<super::entity::EFGIDString>,
        action: EFRuleAction,
        privileges: Vec<EFSystemPrivilege>
    }

    #[derive(Debug)]
    enum EFEntityPrivilege {
        SeeEntity,
        SeeEntityMetadata,
        SeeEntityAttributes,
        ModifyEntity,
        DoEntityActions,
    }

    #[derive(Debug)]
    pub struct EFEntityRule {
        identities: Option<super::entity::EFGIDString>,
        entities: Vec<super::entity::EFGIDString>,
        action: EFRuleAction,
        privileges: Vec<EFEntityPrivilege>
    }
}

#[derive(Debug)]
pub struct EFSystem {
    pub entity_counter: usize,
    pub entities: Vec<entity::EFGIDString>,
    pub subsystems: Vec<entity::EFGIDString>,
    pub system_rules: Vec<rules::EFSystemRule>,
    pub entity_rules: Vec<rules::EFEntityRule>
}
