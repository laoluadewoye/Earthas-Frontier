use crate::utils::result::{EFOk, EFError};

#[derive(Debug, Clone)]
pub struct EFVersion(pub u8, pub u8, pub u8);

impl EFVersion {
    pub fn get_major(&self) -> u8 { self.0 }
    pub fn get_minor(&self) -> u8 { self.1 }
    pub fn get_patch(&self) -> u8 { self.2 }
}

pub trait EFComponent {
    type ComponentParams;

    fn new(params: Self::ComponentParams) -> Self;
    fn build(params: Self::ComponentParams, version: EFVersion) -> Self;
    fn get_component_str(&self) -> String;
    fn get_component_version(&self) -> EFVersion;
    fn clone_component(&self) -> Self;
    fn handle_request(&self, request: &efid::EFQuery) -> efid::EFResponse;
}

// Note: implementations should prefer big endian byte order
#[derive(Debug)]
pub struct EFByteRep {
    pub bytes: Vec<u8>
}

impl EFComponent for EFByteRep {
    type ComponentParams = Vec<u8>;

    fn new(params: Self::ComponentParams) -> Self {
        EFByteRep { bytes: params }
    }

    fn build(params: Self::ComponentParams, _version: EFVersion) -> Self {
        EFByteRep { bytes: params }
    }

    fn get_component_str(&self) -> String {
        // Get component start and start of first attribute
        let (component_start, attr_one_start) = match self.bytes.get(2..4) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return String::from(""); }
        };

        // Return the component string
        match self.bytes.get(component_start..attr_one_start) {
            Some(b) => match String::from_utf8(b.to_vec()) {
                Ok(s) => s,
                Err(_) => String::from("")
            },
            None => String::from("")
        }
    }

    fn get_component_version(&self) -> EFVersion {
        // Get version start and component start
        let (version_start, component_start) = match self.bytes.get(1..3) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return EFVersion(0u8, 0u8, 0u8); }
        };

        // Return the version numbers
        match self.bytes.get(version_start..component_start) {
            Some(b) => EFVersion(b[0], b[1], b[2]),
            None => return EFVersion(0u8, 0u8, 0u8)
        }
    }

    fn clone_component(&self) -> Self {
        EFByteRep { bytes: self.bytes.clone() }
    }

    fn handle_request(&self, request: &efid::EFQuery) -> efid::EFResponse {
        efid::EFResponse
    }
}

pub struct EFByteRepBuilder {
    pub byte_vectors: Vec<Vec<u8>>,
    pub version_vector: Vec<u8>,
    pub component_vector: Vec<u8>
}

pub trait EFByteRepCompatible {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError>;
    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized;
}

pub trait EFByteRepCompatibleEnum {
    fn get_byte_vec(&self) -> Vec<u8>;
    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized;
}

pub mod efid;
pub mod primitives;
pub mod common;
pub mod entity;
pub mod components;
