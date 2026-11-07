use std::hash::Hash;

#[derive(Debug)]
pub enum EFURIAuthority {
    Global,
    Local,
    Connection(String)
}

#[derive(Debug, Clone)]
pub enum EFURITarget {
    ID(String),
    Name(String)
}

impl EFURITarget {
    pub fn to_type_and_string(&self) -> (&str, &String) {
        match self {
            EFURITarget::ID(s) => ("id", s),
            EFURITarget::Name(s) => ("name", s)
        }
    }
}

#[derive(Debug)]
pub enum EFURIPathComponent {
    System(EFURITarget),
    Parent
}

#[derive(Debug)]
pub struct EFURI {
    authority: EFURIAuthority,
    path_from_authority: Vec<EFURIPathComponent>,
    component_fragment: String,
    entity_target: EFURITarget
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EFURIString(pub String);

impl EFURI {
    pub fn to_uri_string(&self) -> EFURIString {
        let mut builder: String = String::new();

        // Add authority
        match &self.authority {
            EFURIAuthority::Global => builder.push_str("$global://"),
            EFURIAuthority::Local => builder.push_str("$local://"),
            EFURIAuthority::Connection(remote_address) => {
                builder.push_str("$connection:");
                builder.push_str(remote_address.as_str());
                builder.push_str("://");
            }
        };

        // Add path
        for path_component in &self.path_from_authority {
            match path_component {
                EFURIPathComponent::Parent => builder.push_str("../"),
                EFURIPathComponent::System(target) => {
                    let (target_type, target_string) = target.to_type_and_string();
                    builder.push_str(target_type);
                    builder.push(':');
                    builder.push_str(target_string.as_str());
                    builder.push('/');
                }
            };
        }

        // Add fragment
        builder.push('#');
        builder.push_str(self.component_fragment.as_str());

        // Add target
        builder.push('?');

        let (et_type, et_string) = self.entity_target.to_type_and_string();
        builder.push_str(et_type);
        builder.push(':');
        builder.push_str(et_string.as_str());

        // Return the string
        EFURIString(builder)
    }
}

#[derive(Debug)]
pub struct EFRequest;
// {
//     sending_entity: EFID,
//     reciving_entity: EFID,
//     query: Vec<String>
// }

#[derive(Debug)]
pub struct EFResponse;
// {
//     sending_entity: EFID,
//     reciving_entity: EFID,
//     response: Vec<EFResult<EFByteRep>>
// }
