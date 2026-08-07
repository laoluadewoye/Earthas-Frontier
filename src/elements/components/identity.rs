use crate::elements::{EFVersion, EFComponent, EFByteRep, EFByteRepCompatible, EFByteRepCompatibleEnum};
use crate::elements::efid::{EFIDEntityOrName, EFQuery, EFResponse};
use crate::utils::result::{EFOk, EFError};
use crate::utils::versions::get_efidentity_version;
use crate::utils::std::{get_byte_rep_from_byte_vectors, get_byte_vectors_from_byte_rep};

#[derive(Debug, Clone)]
enum EFIdentityType {
    User,
    Role
}

impl EFByteRepCompatibleEnum for EFIdentityType where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFIdentityType::User => vec![0u8],
            EFIdentityType::Role => vec![1u8]
        }
    }

    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized {
        let type_byte: u8 = match byte_vec.get(0) {
            Some(tb) => tb.clone(),
            None => {
                return Err(EFError{ 
                    function: String::from("from_byte_vec"), 
                    line: String::from("match byte_vec.get(0)"), 
                    msg: String::from("Could not get type byte for EFIdentityType.")
                });
            }
        };

        match type_byte {
            0u8 if byte_vec.len() == 1 => Ok(EFOk{
                value: EFIdentityType::User, msg: String::from("Returned user.")
            }),
            1u8 if byte_vec.len() == 1 => Ok(EFOk{
                value: EFIdentityType::Role, msg: String::from("Returned role.")
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 1 (inclusive)
                    and without any extra data for EFIdentityType.")
            })
        }
    }
}

#[derive(Debug)]
pub struct EFIdentity {
    name: String,
    identity_type: EFIdentityType,
    secret: Option<EFIDEntityOrName>,
    version: EFVersion
}

impl EFComponent for EFIdentity {
    type ComponentParams = (String, EFIdentityType, Option<EFIDEntityOrName>);

    fn new(params: Self::ComponentParams) -> Self where Self: Sized {
        EFIdentity { 
            name: params.0, identity_type: params.1, secret: params.2, 
            version: get_efidentity_version()
        }
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self where Self: Sized {
        EFIdentity { 
            name: params.0, identity_type: params.1, secret: params.2, 
            version
        }
    }

    fn get_component_str(&self) -> String {
        String::from("identity")
    }

    fn get_component_version(&self) -> EFVersion {
        self.version.clone()
    }

    fn clone_component(&self) -> Self where Self: Sized {
        EFIdentity {
            name: self.name.clone(), 
            identity_type: self.identity_type.clone(), 
            secret: self.secret.clone(), 
            version: self.version.clone()
        }
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFIdentity {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from attributes
        let version_bytes: Vec<u8> = vec![self.version.0, self.version.1, self.version.2];
        let component_bytes: Vec<u8> = self.get_component_str().into_bytes();
        let name_bytes: Vec<u8> = self.name.clone().into_bytes();
        let identity_type_bytes: Vec<u8> = self.identity_type.get_byte_vec();
        let secret_bytes: Vec<u8> = match &self.secret {
            Some(se) => se.clone().get_byte_vec(),
            None => Vec::new()
        };

        // Return byte rep
        let mut byte_vector_set: Vec<Vec<u8>> = vec![
            version_bytes, component_bytes, name_bytes, identity_type_bytes, secret_bytes
        ];
        get_byte_rep_from_byte_vectors(&mut byte_vector_set)
    }

    fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the vectors for each attribute
        let identity_bytes: Vec<Vec<u8>> = match get_byte_vectors_from_byte_rep(br) {
            Ok(bv) => bv.value,
            Err(e) => { return Err(e); }
        };

        // Check the component
        let component: String = match String::from_utf8(identity_bytes[0].clone()) {
            Ok(s) => s,
            Err(_) => {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("String::from_utf8(identity_bytes[0].clone())"), 
                    msg: String::from("Passed in byte rep is not compatible with UTF-8 for.")
                });
            }
        };

        if !component.eq("identity") {
            return Err(EFError{
                function: String::from("from_byte_rep"), 
                line: String::from("!br.component.eq(\"identity\")"), 
                msg: String::from("Component is not set to identity.")
            });
        }

        // Create the attributes
        let version: EFVersion = match identity_bytes[1].len() == 3 {
            true => EFVersion(identity_bytes[1][0], identity_bytes[1][1], identity_bytes[1][2]),
            false => {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("identity_bytes[1].len() == 3"), 
                    msg: String::from("Could not parse version for identity.")
                });
            }
        };
        let name: String = match String::from_utf8(identity_bytes[2].clone()) {
            Ok(s) => s,
            Err(_) => {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("String::from_utf8(name_bytes)"), 
                    msg: String::from("Passed in byte rep is not compatible with UTF-8 for identity's name.")
                });
            }
        };
        let identity_type: EFIdentityType = match EFIdentityType::from_byte_vec(&identity_bytes[3]) {
            Ok(id) => id.value,
            Err(e) => { return Err(e); }
        };
        let secret: Option<EFIDEntityOrName> = match identity_bytes[4].is_empty() {
            true => None,
            false => match EFIDEntityOrName::from_byte_vec(&identity_bytes[4]) {
                Ok(eon) => Some(eon.value),
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("EFIDEntityOrName::from_byte_vec(&identity_bytes[4])"), 
                        msg: String::from("Passed in byte rep is not compatible with UTF-8 for identity's secret.")
                    });
                }
            }
        };

        // Create the final product if everything passes
        Ok(EFOk{
            value: EFIdentity::build((name, identity_type, secret), version),
            msg: String::from("Converted the byte rep into an identity.")
        })
    }
}
