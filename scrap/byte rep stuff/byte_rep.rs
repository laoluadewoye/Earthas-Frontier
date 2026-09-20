use std::ops::Range;
use crate::utils::constants::{EMPTY_VERSION, EMPTY_STR_SLICE};
use crate::utils::result::{EFValueResult, EFReturnEvent};
use crate::utils::vector::get_generic_vec_obj;
use super::EFVersion;

// Constants for creating byte representations
const ATTR_COUNT_INDEX: usize = 0; // Number of attributes in bytes

const VERSION_START_INDEX: usize = 1; // Start of the version number
const TYPE_START_INDEX: usize = 2; // Start of the type string
const BYTE_REP_METADATA_COUNT: usize = 2; // Version and type type

const FIRST_ATTR_START_INDEX: usize = BYTE_REP_METADATA_COUNT + 1; // Start of the first attribute

const BYTE_REP_EMPTY_ENCODING: usize = 0; // For when an attribute contains an "empty" value (i.e. None)
const BYTE_REP_EMPTY_ENCODING_VALUE: u8 = 0u8; // The value to fill in for an empty encoding

#[derive(Debug, Clone)]
pub struct EFByteRep(Vec<u8>);

impl EFByteRep {
    pub fn get_element_version(&self) -> EFVersion {
        // Get version start and type start
        let vt_start_range: Range<usize> = VERSION_START_INDEX..TYPE_START_INDEX+1;
        let (version_start_num, type_start_num): (usize, usize) = match self.0.get(vt_start_range) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return EMPTY_VERSION; }
        };

        // Return the version numbers
        match self.0.get(version_start_num..type_start_num) {
            Some(b) => EFVersion(b[0], b[1], b[2]),
            None => EMPTY_VERSION
        }
    }

    pub fn get_element_type(&self) -> &str {
        // Get type start and start of first attribute
        let tf_start_range: Range<usize> = TYPE_START_INDEX..FIRST_ATTR_START_INDEX+1;
        let (type_start_num, first_attr_start_num) = match self.0.get(tf_start_range) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return EMPTY_STR_SLICE; }
        };

        // Return the type string
        match self.0.get(type_start_num..first_attr_start_num) {
            Some(b) => match str::from_utf8(b) {
                Ok(s) => s,
                Err(_) => EMPTY_STR_SLICE
            },
            None => EMPTY_STR_SLICE
        }
    }
}

pub fn byte_rep_from_attrs(attr_vectors: Vec<Vec<u8>>) -> EFValueResult<EFByteRep> {
    // Get the number of attributes
    let attr_count: usize = attr_vectors.len();

    // Get the lengths of each vector
    let attr_lengths: Vec<usize> = attr_vectors.iter().map(|a_v| a_v.len()).collect();

    // Create the offsets for each attribute
    let attr_offsets: Vec<u8> = Vec::new();
    let current_offset: u8 = 

}

pub fn byte_rep_from_component(
    br_version: &EFVersion, 
    br_type: &str, 
    attr_vectors: Vec<Vec<u8>>
) -> EFValueResult<EFByteRep> {
    // Create version and type vectors
    let version_vector: Vec<u8> = vec![br_version.0, br_version.1, br_version.2];
    let type_vector: Vec<u8> = br_type.to_string().into_bytes();

    // Assemble a byte vector in order
    let mut byte_vectors: Vec<Vec<u8>> = vec![version_vector, type_vector];
    byte_vectors.append(&mut attr_vectors);

    // Pass to general byte rep function
    byte_rep_from_attrs(byte_vectors)
}

#[derive(Debug, Clone)]
pub struct EFByteRepBuilder {
    pub version_vector: Vec<u8>,
    pub type_vector: Vec<u8>,
    pub byte_vectors: Vec<Vec<u8>>
}

impl EFByteRepBuilder {
    pub fn create_byte_rep(&mut self) -> EFValueResult<EFByteRep> {
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
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "create_byte_rep", 
                        "Got a bad index for byte vector set."
                    ));
                }
            };
            bytes.append(&mut byte_vector);
        }

        // Return byte rep
        Ok(EFByteRep(bytes))
    }

    // Note: function needs to be rethought because the offset logic is pretty naive
    //      In the case where one index is normal and the next is an empty encoding,
    //      how does the program know to handle that?
    pub fn from_byte_rep(byte_rep: &EFByteRep) -> EFValueResult<EFByteRepBuilder> {
        // Get the attribute count
        let attribute_count: usize = match get_generic_vec_obj(&byte_rep.0, 0) {
            Ok(v) => v as usize,
            Err(e) => {
                e.add_func_info_log("from_byte_rep", "Could not create byte rep.");
                return Err(e);
            }
        };

        // One loop to grab the offsets
        let mut offsets: Vec<usize> = Vec::new();
        for i in 0..attribute_count {
            offsets.push(byte_rep.0[1+i] as usize);
        }

        // // Create empty byte vector set
        // let mut byte_vectors: Vec<Vec<u8>> = Vec::new();

        // // Another loop to get the byte vectors
        // for i in 0..attribute_count {
        //     // Setup the offset edges
        //     let vector_range: Range<usize> = match i < attribute_count - 1 {
        //         true => offsets[i]..offsets[i+1],
        //         false => offsets[i]..byte_rep.0.len()
        //     };

        //     // Check if the offset is set to the None encoding
        //     if offsets[i] == BYTE_REP_EMPTY_ENCODING {
        //         byte_vectors.push(Vec::new());
        //     }
        //     else if let Some(v) = byte_rep.0.get(vector_range) {
        //         byte_vectors.push(v.to_vec());
        //     }
        //     else {
        //         return Err(EFReturnEvent::new_with_func_info_log(
        //             "from_byte_rep", 
        //             "Got a bad index for byte rep."
        //         ));
        //     }
        // }

        // Deconstruct vectors into a builder
        let version_vector: Vec<u8> = byte_vectors.remove(0);
        let type_vector: Vec<u8> = byte_vectors.remove(0);

        Ok(EFOk{
            value: EFByteRepBuilder { version_vector, type_vector, byte_vectors },
            msg: String::from("Created byte rep builder.")
        })
    }

    pub fn validate_component_type(&self, component_type: &str) -> EFResult<EFSuccess> {
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

    pub fn get_version(&self) -> EFResult<EFVersion> {
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
    ) -> EFResult<(EFVersion, Vec<Vec<u8>>)> {
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
    fn to_byte_rep(&self) -> EFResult<EFByteRep>;
    fn from_byte_rep(byte_rep: &EFByteRep) -> EFResult<Self> where Self: Sized;
}

pub trait EFByteVecCompatible {
    fn to_byte_vec(&self) -> EFResult<Vec<u8>>;
    fn from_byte_vec(byte_vector: &Vec<u8>) -> EFResult<Self> where Self: Sized;
}

pub mod enum_helper {
    use super::*;

    pub fn get_byte_vector_from_enum_and_string(enum_byte: u8, enum_str: &String) -> Vec<u8> {
        let mut byte_vector: Vec<u8> = vec![enum_byte];
        let mut enum_str_vec: Vec<u8> = enum_str.clone().into_bytes();
        byte_vector.append(&mut enum_str_vec);
        byte_vector
    }

    pub fn get_enum_and_string_from_byte_vector(byte_vector: &Vec<u8>) -> EFResult<(u8, String)> {
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
