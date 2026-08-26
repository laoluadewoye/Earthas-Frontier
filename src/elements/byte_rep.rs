use std::ops::Range;
use crate::utils::constants::EMPTY_STR_SLICE;
use crate::utils::result::*;
use crate::utils::vector::*;
use super::{EFVersion};

// Constants for creating byte representations
const ATTR_COUNT_INDEX: usize = 0; // Number of attributes in bytes
const VERSION_START_INDEX: usize = 1; // Start of the version number
const COMPONENT_START_INDEX: usize = 2; // Start of the component string
const FIRST_ATTR_START_INDEX: usize = 3; // Start of the first attribute
const BYTE_REP_METADATA_COUNT: usize = 2; // Version and component type
const BYTE_REP_EMPTY_ENCODING: usize = 0; // For when an attribute contains an "empty" value (i.e. None)
const BYTE_REP_EMPTY_ENCODING_VALUE: u8 = 0u8; // The value to fill in for an empty encoding

#[derive(Debug, Clone)]
pub struct EFByteRep(Vec<u8>);

impl EFByteRep {
    pub fn get_component_version(&self) -> EFVersion {
        // Get version start and component start
        let vc_start_range: Range<usize> = VERSION_START_INDEX..COMPONENT_START_INDEX+1;
        let (version_start_num, component_start_num): (usize, usize) = match self.0.get(vc_start_range) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return EFVersion(0u8, 0u8, 0u8); }
        };

        // Return the version numbers
        match self.0.get(version_start_num..component_start_num) {
            Some(b) => EFVersion(b[0], b[1], b[2]),
            None => return EFVersion(0u8, 0u8, 0u8)
        }
    }

    pub fn get_component_type(&self) -> &str {
        // Get component start and start of first attribute
        let cf_start_range: Range<usize> = COMPONENT_START_INDEX..FIRST_ATTR_START_INDEX+1;
        let (component_start_num, first_attr_start_num) = match self.0.get(cf_start_range) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return EMPTY_STR_SLICE; }
        };

        // Return the component string
        match self.0.get(component_start_num..first_attr_start_num) {
            Some(b) => match str::from_utf8(b) {
                Ok(s) => s,
                Err(_) => EMPTY_STR_SLICE
            },
            None => EMPTY_STR_SLICE
        }
    }
}

#[derive(Debug, Clone)]
pub struct EFByteRepBuilder {
    pub version_vector: Vec<u8>,
    pub type_vector: Vec<u8>,
    pub byte_vectors: Vec<Vec<u8>>
}

impl EFByteRepBuilder {
    pub fn create_byte_rep(&mut self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create a new bytes vector
        let attribute_count: usize = self.byte_vectors.len() + BYTE_REP_METADATA_COUNT;
        let mut bytes: Vec<u8> = vec![attribute_count as u8];

        // Create a length vector
        let mut lengths: Vec<usize> = vec![self.version_vector.len(), self.type_vector.len()];
        let mut byte_lengths: Vec<usize> = self.byte_vectors.iter().map(|v| v.len()).collect();
        lengths.append(&mut byte_lengths);

        // Use lengths to create offsets
        let mut cur_offset: u8 = self.byte_vectors.len() as u8;
        for i in 0..self.byte_vectors.len() {
            match lengths[i] {
                BYTE_REP_EMPTY_ENCODING => { bytes.push(BYTE_REP_EMPTY_ENCODING_VALUE); },
                _ => { bytes.push(cur_offset); }
            }
            cur_offset = cur_offset + lengths[i] as u8;
        }

        // Add metadata
        bytes.append(&mut self.version_vector);
        bytes.append(&mut self.type_vector);

        // Add data
        for i in 0..self.byte_vectors.len() {
            let mut byte_vector: &mut Vec<u8> = match self.byte_vectors.get_mut(i) {
                Some(bv) => bv,
                None => {
                    return Err(EFError{
                        function: String::from("create_byte_rep"), 
                        line: String::from("self.byte_vectors.get_mut(i)"), 
                        msg: format!("Got a bad index for byte vector set.")
                    });
                }
            };
            bytes.append(&mut byte_vector);
        }

        // Return byte rep
        Ok(EFOk{
            value: EFByteRep(bytes),
            msg: String::from("Created byte rep.")
        })
    }

    pub fn new_from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<EFByteRepBuilder>, EFError> {
        // Get the attribute count
        let attribute_count: usize = match get_index_from_generic_vector(&byte_rep.0, 0) {
            Ok(a) => a.value as usize,
            Err(e) => { return Err(e); }
        };

        // One loop to grab the offsets
        let mut offsets: Vec<usize> = Vec::new();
        for i in 0..attribute_count {
            offsets.push(byte_rep.0[1+i] as usize);
        }

        // Create empty byte vector set
        let mut byte_vectors: Vec<Vec<u8>> = Vec::new();

        // Another loop to get the byte vectors
        for i in 0..attribute_count {
            // Setup the offset edges
            let vector_range: Range<usize> = match i < attribute_count - 1 {
                true => offsets[i]..offsets[i+1],
                false => offsets[i]..byte_rep.0.len()
            };

            // Check if the offset is set to the None encoding
            if offsets[i] == BYTE_REP_EMPTY_ENCODING {
                byte_vectors.push(Vec::new());
            }
            else if let Some(v) = byte_rep.0.get(vector_range) {
                byte_vectors.push(v.to_vec());
            }
            else {
                return Err(EFError{
                    function: String::from("new_from_byte_rep"), 
                    line: String::from("byte_rep.0.get(vector_range)"), 
                    msg: format!("Got a bad index for byte rep.")
                });
            }
        }

        // Deconstruct vectors into a builder
        let version_vector: Vec<u8> = byte_vectors.remove(0);
        let type_vector: Vec<u8> = byte_vectors.remove(0);

        Ok(EFOk{
            value: EFByteRepBuilder { version_vector, type_vector, byte_vectors },
            msg: String::from("Created byte rep builder.")
        })
    }

    pub fn validate_component_type(&self, component_type: &str) -> Result<EFOk<EFSuccess>, EFError> {
        match get_string_from_byte_vector(&self.type_vector) {
            Ok(test_type) => match test_type.value.eq(component_type) {
                true => Ok(EFOk{value: EFSuccess, msg: format!("Component is type of {}.", component_type)}),
                false => Err(EFError{
                    function: String::from("validate_component_type"),
                    line: String::from("test_type.value.eq(component_type)"),
                    msg: format!("Component is not type of {}.", component_type)
                })
            },
            Err(e) => Err(e)
        }
    }

    pub fn get_version(&self) -> Result<EFOk<EFVersion>, EFError> {
        match self.version_vector.len() == 3 {
            true => Ok(EFOk{
                value: EFVersion(self.version_vector[0], self.version_vector[1], self.version_vector[2]),
                msg: String::from("Created version from builder.")
            }),
            false => Err(EFError{
                function: String::from("get_version"),
                line: String::from("self.version_vector.len() == 3"),
                msg: String::from("Could not create version from builder.")
            })
        }
    }

    pub fn validate_br_for_ver_and_attrs(
        byte_rep: &EFByteRep, component_type: &str
    ) -> Result<EFOk<(EFVersion, Vec<Vec<u8>>)>, EFError> {
        // Get the builder and validate it
        let builder: EFByteRepBuilder = match EFByteRepBuilder::new_from_byte_rep(byte_rep) {
            Ok(b) => b.value,
            Err(e) => { return Err(e); }
        };

        // Validate that the component type matches
        if let Err(e) = builder.validate_component_type(component_type) {
            return Err(e);
        }

        // Get the version
        let version: EFVersion = match builder.get_version() {
            Ok(v) => v.value,
            Err(e) => { return Err(e); }
        };

        // Return tuple
        Ok(EFOk{ 
            value: (version, builder.byte_vectors), 
            msg: format!("Returned version and vectors for {}.", component_type)
        })
    }
}

pub trait EFByteRepCompatible {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError>;
    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized;
}

pub trait EFByteVecCompatible {
    fn to_byte_vec(&self) -> Result<EFOk<Vec<u8>>, EFError>;
    fn from_byte_vec(byte_vector: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized;
}

pub mod enum_helper {
    use super::*;

    pub fn get_byte_vector_from_enum_and_string(enum_byte: u8, enum_str: &String) -> Vec<u8> {
        let mut byte_vector: Vec<u8> = vec![enum_byte];
        let mut enum_str_vec: Vec<u8> = enum_str.clone().into_bytes();
        byte_vector.append(&mut enum_str_vec);
        byte_vector
    }

    pub fn get_enum_and_string_from_byte_vector(byte_vector: &Vec<u8>) -> Result<EFOk<(u8, String)>, EFError> {
        let enum_byte: u8 = match get_index_from_generic_vector(byte_vector, 0) {
            Ok(index_object) => index_object.value,
            Err(e) => { return Err(e); }
        };

        if byte_vector.len() == 1 {
            Ok(EFOk{
                value: (enum_byte, String::from("")), 
                msg: String::from("Returned type byte and empty string.")
            })
        }
        else {
            match get_index_range_from_generic_vector(byte_vector, Some(1), None) {
                Ok(index_range) => match get_string_from_byte_vector(&index_range.value) {
                    Ok(s) => Ok(EFOk{
                        value: (enum_byte, s.value.to_string()),
                        msg: String::from("Returned type byte and non-empty string.")
                    }),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        }
    }
}
