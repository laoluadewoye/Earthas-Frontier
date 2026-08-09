use crate::elements::{EFOk, EFError, EFByteRep, EFByteRepCompatibleEnum};
use crate::utils::generic_vector::{get_index_from_generic_vector, get_index_range_from_generic_vector};
use crate::utils::byte_vector::{get_byte_vector_from_enum_and_string, get_enum_and_string_from_byte_vector};

#[derive(Debug)]
pub enum EFIDAuthority {
    Global,
    Local,
    Connection(String)
}

impl EFByteRepCompatibleEnum for EFIDAuthority where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFIDAuthority::Global => vec![0u8],
            EFIDAuthority::Local => vec![1u8],
            EFIDAuthority::Connection(type_str) => {
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
                value: EFIDAuthority::Global, 
                msg: String::from("Returned global.")
            }),
            1u8 if type_str.is_empty() => Ok(EFOk{
                value: EFIDAuthority::Local, msg: String::from("Returned local.")
            }),
            2u8 if !type_str.is_empty() => Ok(EFOk{
                value: EFIDAuthority::Connection(type_str), 
                msg: String::from("Returned connection.") 
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 2 (inclusive)
                    and only return a string for connection for EFIDAuthority.")
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum EFIDEntityOrName {
    ID(String),
    Name(String)
}

impl EFByteRepCompatibleEnum for EFIDEntityOrName where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFIDEntityOrName::ID(type_str) => {
                get_byte_vector_from_enum_and_string(0u8, type_str)
            },
            EFIDEntityOrName::Name(type_str) => {
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
                value: EFIDEntityOrName::ID(type_str), 
                msg: String::from("Returned id.")
            }),
            1u8 if !type_str.is_empty() => Ok(EFOk{
                value: EFIDEntityOrName::Name(type_str), 
                msg: String::from("Returned name.")
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 1 (inclusive)
                    and must return a string for EFIDEntityOrName.")
            })
        }
    }
}

#[derive(Debug)]
pub enum EFIDPathComponent {
    System(EFIDEntityOrName),
    Parent
}

impl EFByteRepCompatibleEnum for EFIDPathComponent where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFIDPathComponent::System(type_enum) => {
                let mut byte_vec: Vec<u8> = vec![0u8];
                let mut type_vec: Vec<u8> = type_enum.get_byte_vec();
                byte_vec.append(&mut type_vec);
                byte_vec
            },
            EFIDPathComponent::Parent => vec![1u8]
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
                    Ok(index_range) => match EFIDEntityOrName::from_byte_vec(&index_range.value) {
                        Ok(eon) => Ok(EFOk{
                            value: EFIDPathComponent::System(eon.value), 
                            msg: String::from("Returned system.")
                        }),
                        Err(e) => Err(e)
                    },
                    Err(e) => Err(e)
                }
            },
            1u8 if byte_vec.len() == 1 => Ok(EFOk{
                value: EFIDPathComponent::Parent, msg: String::from("Returned parent.")
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 1 (inclusive)
                    and must return data only for System for EFIDPathComponent.")
            })
        }
    }
}

#[derive(Debug)]
pub struct EFID {
    authority: EFIDAuthority,
    path_from_authority: Vec<EFIDPathComponent>,
    component_fragment: String,
    entity: EFIDEntityOrName
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
