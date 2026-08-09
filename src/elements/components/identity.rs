use crate::elements::{EFVersion, EFComponent, EFByteRep, EFByteRepBuilder, EFByteRepCompatible, EFByteRepCompatibleEnum};
use crate::elements::efid::{EFIDEntityOrName, EFQuery, EFResponse};
use crate::utils::result::{EFOk, EFError};
use crate::utils::versions::EFIDENTITY_VERSION;
use crate::utils::component_str::EFIDENTITY_STR;
use crate::utils::generic_vector::get_index_from_generic_vector;
use crate::utils::byte_vector::{
    get_string_from_byte_vector,
    get_byte_rep_from_builder, 
    get_byte_vectors_and_version_from_byte_rep
};

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
        let type_byte: u8 = match get_index_from_generic_vector(byte_vec, 0) {
            Ok(index_object) => index_object.value,
            Err(e) => { return Err(e); }
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
            version: EFIDENTITY_VERSION
        }
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self where Self: Sized {
        EFIdentity { 
            name: params.0, identity_type: params.1, secret: params.2, 
            version
        }
    }

    fn get_component_str(&self) -> String {
        String::from(EFIDENTITY_STR)
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
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.version.0, self.version.1, self.version.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let name_bytes: Vec<u8> = self.name.clone().into_bytes();
        let identity_type_bytes: Vec<u8> = self.identity_type.get_byte_vec();
        let secret_bytes: Vec<u8> = match &self.secret {
            Some(se) => se.clone().get_byte_vec(),
            None => Vec::new()
        };
        let byte_vectors: Vec<Vec<u8>> = vec![
            name_bytes, identity_type_bytes, secret_bytes
        ];

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder{ 
            byte_vectors, version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFIDENTITY_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the attributes
        let name: String = match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(index_object) => match get_string_from_byte_vector(index_object.value) {
                Ok(s) => s.value,
                Err(e) => { return Err(e); }
            },
            Err(e) => { return Err(e); }
        };
        let identity_type: EFIdentityType = match get_index_from_generic_vector(&byte_vectors, 1) {
            Ok(index_object) => match EFIdentityType::from_byte_vec(&index_object.value) {
                Ok(id) => id.value,
                Err(e) => { return Err(e); }
            },
            Err(e) => { return Err(e); }
        };
        let secret: Option<EFIDEntityOrName> = match get_index_from_generic_vector(&byte_vectors, 2) {
            Ok(index_object) => match index_object.value.is_empty() {
                true => None,
                false => match EFIDEntityOrName::from_byte_vec(&index_object.value) {
                    Ok(eon) => Some(eon.value),
                    Err(e) => { return Err(e); }
                }
            },
            Err(e) => { return Err(e); }
        };

        // Create the final product if everything passes
        Ok(EFOk{
            value: EFIdentity::build((name, identity_type, secret), version),
            msg: String::from("Converted the byte rep into an identity.")
        })
    }
}
