use crate::elements::{EFOk, EFError};
use crate::elements::byte_rep::{EFByteRep, EFByteRepCompatibleEnum};
use crate::utils::generic_vector::{get_index_from_generic_vector, get_index_range_from_generic_vector};
use crate::utils::byte_vector::{get_byte_vector_from_enum_and_string, get_enum_and_string_from_byte_vector};

#[derive(Debug)]
pub enum EFURIAuthority {
    Global,
    Local,
    Connection(String)
}

impl EFByteRepCompatibleEnum for EFURIAuthority where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFURIAuthority::Global => vec![0u8],
            EFURIAuthority::Local => vec![1u8],
            EFURIAuthority::Connection(type_str) => {
                get_byte_vector_from_enum_and_string(2u8, type_str)
            }
        }
    }

    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized {
        let (type_byte, type_str) = match get_enum_and_string_from_byte_vector(byte_vec) {
            Ok(res_tuple) => res_tuple.value,
            Err(e) => { return Err(e); }
        };

        match type_byte {
            0u8 if type_str.is_empty() => Ok(EFOk{
                value: EFURIAuthority::Global, 
                msg: String::from("Returned global.")
            }),
            1u8 if type_str.is_empty() => Ok(EFOk{
                value: EFURIAuthority::Local, msg: String::from("Returned local.")
            }),
            2u8 if !type_str.is_empty() => Ok(EFOk{
                value: EFURIAuthority::Connection(type_str), 
                msg: String::from("Returned connection.") 
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 2 (inclusive)
                    and only return a string for connection for EFURIAuthority.")
            })
        }
    }
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

impl EFByteRepCompatibleEnum for EFURITarget where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFURITarget::ID(type_str) => {
                get_byte_vector_from_enum_and_string(0u8, type_str)
            },
            EFURITarget::Name(type_str) => {
                get_byte_vector_from_enum_and_string(1u8, type_str)
            }
        }
    }

    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized {
        let (type_byte, type_str) = match get_enum_and_string_from_byte_vector(byte_vec) {
            Ok(res_tuple) => res_tuple.value,
            Err(e) => { return Err(e); }
        };

        match type_byte {
            0u8 if !type_str.is_empty() => Ok(EFOk{
                value: EFURITarget::ID(type_str), 
                msg: String::from("Returned id.")
            }),
            1u8 if !type_str.is_empty() => Ok(EFOk{
                value: EFURITarget::Name(type_str), 
                msg: String::from("Returned name.")
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 1 (inclusive)
                    and must return a string for EFURITarget.")
            })
        }
    }
}

#[derive(Debug)]
pub enum EFURIPathComponent {
    System(EFURITarget),
    Parent
}

impl EFByteRepCompatibleEnum for EFURIPathComponent where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFURIPathComponent::System(type_enum) => {
                let mut byte_vec: Vec<u8> = vec![0u8];
                let mut type_vec: Vec<u8> = type_enum.get_byte_vec();
                byte_vec.append(&mut type_vec);
                byte_vec
            },
            EFURIPathComponent::Parent => vec![1u8]
        }
    }

    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized {
        let type_byte: u8 = match get_index_from_generic_vector(byte_vec, 0) {
            Ok(index_object) => index_object.value,
            Err(e) => { return Err(e); }
        };

        match type_byte {
            0u8 => {
                match get_index_range_from_generic_vector(byte_vec, Some(1), None) {
                    Ok(index_range) => match EFURITarget::from_byte_vec(&index_range.value) {
                        Ok(eon) => Ok(EFOk{
                            value: EFURIPathComponent::System(eon.value), 
                            msg: String::from("Returned system.")
                        }),
                        Err(e) => Err(e)
                    },
                    Err(e) => Err(e)
                }
            },
            1u8 if byte_vec.len() == 1 => Ok(EFOk{
                value: EFURIPathComponent::Parent, msg: String::from("Returned parent.")
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 1 (inclusive)
                    and must return data only for System for EFURIPathComponent.")
            })
        }
    }
}

#[derive(Debug)]
pub struct EFURI {
    authority: EFURIAuthority,
    path_from_authority: Vec<EFURIPathComponent>,
    component_fragment: String,
    entity_target: EFURITarget
}

#[derive(Debug, Clone)]
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
pub struct EFQuery;
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
//     response: Vec<Result<EFOk<EFByteRep>, EFError>>
// }
