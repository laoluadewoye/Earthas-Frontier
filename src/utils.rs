pub mod result {
    #[derive(Debug)]
    pub struct EFSuccess; //Means the function successfully ran without glaring issues

    #[derive(Debug)]
    pub struct EFOk<T> {
        pub value: T,
        pub msg: String
    }

    #[derive(Debug)]
    pub struct EFError {
        pub function: String,
        pub line: String,
        pub msg: String
    }

    impl EFError {
        pub fn to_string(&self) -> String {
            format!("{:?}", self)
        }
    }
}

pub mod component_str {
    // Unsigned integers
    pub const EFUSIZE_STR: &'static str = "usize";
    pub const EFU8_STR: &'static str = "u8";
    pub const EFU16_STR: &'static str = "u16";
    pub const EFU32_STR: &'static str = "u32";
    pub const EFU64_STR: &'static str = "u64";
    pub const EFU128_STR: &'static str = "u128";

    // Signed integers
    pub const EFISIZE_STR: &'static str = "isize";
    pub const EFI8_STR: &'static str = "i8";
    pub const EFI16_STR: &'static str = "i16";
    pub const EFI32_STR: &'static str = "i32";
    pub const EFI64_STR: &'static str = "i64";
    pub const EFI128_STR: &'static str = "i128";

    // Signed floats
    pub const EFF32_STR: &'static str = "f32";
    pub const EFF64_STR: &'static str = "f64";

    // Other primitives
    pub const EFBOOL_STR: &'static str = "bool";
    pub const EFCHAR_STR: &'static str = "char";

    // Common components
    pub const EFSTRING_STR: &'static str = "string";

    // Core components
    pub const EFIDENTITY_STR: &'static str = "identity";
    pub const EFSECRET_STR: &'static str = "secret";
}

pub mod versions {
    use crate::elements::EFVersion;
    
    // Unsigned integers
    pub const EFUSIZE_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFU8_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFU16_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFU32_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFU64_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFU128_VERSION: EFVersion = EFVersion(0, 0, 1);

    // Signed integers
    pub const EFISIZE_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFI8_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFI16_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFI32_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFI64_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFI128_VERSION: EFVersion = EFVersion(0, 0, 1);

    // Signed floats
    pub const EFF32_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFF64_VERSION: EFVersion = EFVersion(0, 0, 1);

    // Other primitives
    pub const EFBOOL_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFCHAR_VERSION: EFVersion = EFVersion(0, 0, 1);

    // Common components
    pub const EFSTRING_VERSION: EFVersion = EFVersion(0, 0, 1);

    // Core components
    pub const EFIDENTITY_VERSION: EFVersion = EFVersion(0, 0, 1);
    pub const EFSECRET_VERSION: EFVersion = EFVersion(0, 0, 1);
}

pub mod json {
    use super::result::{EFOk, EFError};
    use std::{io::Read, path::Path};
    use std::fs::File;
    use serde_json::{Value as JSONValue, from_str as json_from_str};

    pub fn load_json_from_file(file_str: &String) -> Result<EFOk<JSONValue>, EFError> {
        // Create a path
        let file_path: &Path = Path::new(file_str.as_str());

        // Open the file
        let mut file_obj: File = match File::open(file_path) {
            Err(_) => {
                return Err(EFError{
                    function: String::from("load_json_from_file"), 
                    line: String::from("File::open(file_path)"), 
                    msg: format!("Could not open {}.", file_str)
                });
            },
            Ok(f) => f
        };

        // Read the file to string
        let mut file_str: String = String::new();
        if file_obj.read_to_string(&mut file_str).is_err() {
            return Err(EFError{
                function: String::from("load_json_from_file"), 
                line: String::from("file_obj.read_to_string(&mut file_str)"), 
                msg: format!("Could not read {}.", file_str)
            });
        }

        // Parse JSON from string
        let file_json: JSONValue = match json_from_str(file_str.as_str()) {
            Err(_) => {
                return Err(EFError{
                    function: String::from("load_json_from_file"), 
                    line: String::from("json_from_str(file_str.as_str())"), 
                    msg: format!("Could not parse {}.", file_str)
                });
            },
            Ok(j) => j
        };

        // Return the JSON value
        Ok(EFOk{ value: file_json, msg: format!("Parsed {}.", file_str)})
    }
}

pub mod os {
    use super::result::{EFSuccess, EFOk, EFError};
    use std::env::consts::OS;
    use std::env::var as env_var;
    use std::path::Path;
    use std::fs::{create_dir, create_dir_all};

    pub fn get_os_default_folder() -> Result<EFOk<String>, EFError> {
        match OS {
            "windows" => {
                match env_var("APPDATA") {
                    Ok(v) => Ok(EFOk{ value: v, msg: String::from("Returning value of APPDATA") }),
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("get_os_default_folder"), 
                            line: String::from("env_var(\"APPDATA\")"), 
                            msg: format!("Could not find the default application folder for Windows. 
                                Please set APPDATA.")
                        });
                    }
                }
            },
            "linux" | "macos" => {
                match env_var("HOME") {
                    Ok(v) => Ok(EFOk{ value: v, msg: String::from("Returning value of HOME") }),
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("get_os_default_folder"), 
                            line: String::from("env_var(\"HOME\")"), 
                            msg: format!("Could not find the user's home directory for Linux\\Mac OS. 
                                Please set HOME.")
                        });
                    }
                }
            },
            _ => panic!("Other operating systems aren't supported.")
        }
    }

    pub fn create_folder(path_str: &String, cfinp: &String) -> Result<EFOk<EFSuccess>, EFError> {
        let path: &Path = Path::new(path_str.as_str());
        match path.is_dir() {
            false => {
                match cfinp.to_lowercase().as_str() {
                    "true" | "yes" => {
                        match create_dir_all(path) {
                            Ok(_) => Ok(EFOk{ value: EFSuccess, msg: format!("Created all directories for {}.", path_str) }),
                            Err(_) => Err(EFError{
                                function: String::from("create_folder"), 
                                line: String::from("create_dir_all(path)"), 
                                msg: format!("Unable to create all directories for {}.", path_str)
                            })
                        }
                    },
                    "false" | "no" | "" => {
                        match create_dir(path) {
                            Ok(_) => Ok(EFOk{ value: EFSuccess, msg: format!("Created directory for {}.", path_str) }),
                            Err(_) => Err(EFError{
                                function: String::from("create_folder"), 
                                line: String::from("create_dir(path)"), 
                                msg: format!("Unable to create directory for {}.", path_str)
                            })
                        }
                    },
                    _ => Err(EFError{
                        function: String::from("create_folder"), 
                        line: String::from("cfinp.to_lowercase().as_str()"), 
                        msg: format!("An incorrect value was used for create_folder_if_no_parent key.")
                    })
                }
            }
            true => Ok(EFOk{ value: EFSuccess, msg: format!("Directory for {} already exists.", path_str) })
        }
    }
}

pub mod general {
    use super::result::{EFOk, EFError};
    use sha2::{Digest, Sha256, Sha512};

    pub fn get_hash(string_vec: Vec<&String>, entity_hash: &String) -> Result<EFOk<String>, EFError> {
        let hash_bytes: Vec<u8> = match entity_hash.to_lowercase().as_str() {
            "sha256" | "" => {
                let mut hasher: Sha256 = Sha256::new();
                for s in string_vec {
                    Digest::update(&mut hasher, s.as_bytes());
                }
                hasher.finalize().to_vec()
            },
            "sha512" => {
                let mut hasher: Sha512 = Sha512::new();
                for s in string_vec {
                    Digest::update(&mut hasher, s.as_bytes());
                }
                hasher.finalize().to_vec()
            },
            _ => {
                return Err(EFError{
                    function: String::from("get_hash"), 
                    line: String::from("entity_hash.to_lowercase().as_str()"), 
                    msg: format!("An incorrect value was used for entity_hash key.")
                });
            }
        };

        match String::from_utf8(hash_bytes) {
            Ok(h) => Ok(EFOk { value: h, msg: String::from("Created hash.") }),
            Err(_) => Err(EFError{
                function: String::from("get_hash"), 
                line: String::from("String::from_utf8(hash_bytes)"), 
                msg: format!("get_hash failed to make a string from a vector.")
            })
        }
    }
}

pub mod generic_vector {
    use super::result::{EFOk, EFError};

    pub fn get_index_from_generic_vector<T: Clone>(v: &Vec<T>, i: usize) -> Result<EFOk<T>, EFError> {
        match v.get(i) {
            Some(i_v) => Ok(EFOk { 
                value: i_v.clone(), 
                msg: format!("Cloned object at index {}", i)
            }),
            None => Err(EFError {
                function: String::from("get_index_from_generic_vector"),
                line: String::from("v.get(i)"), 
                msg: format!("Could not get object at index {}", i)
            })
        }
    }

    pub fn get_index_range_from_generic_vector<T: Clone>(
        v: &Vec<T>, 
        start: Option<usize>, 
        end: Option<usize>
    ) -> Result<EFOk<Vec<T>>, EFError> {
        let (r, s, e) = match (start, end) {
            // Get only between the bounds
            (Some(s), Some(e)) => (s..e, s, e),

            // Get the rest of index after start
            (Some(s), None) => (s..v.len(), s, v.len()),

            // Get beginning of index up to end
            (None, Some(e)) => (0..e, 0, e),

            // Why would you do this
            (None, None) => {
                return Err(EFError {
                    function: String::from("get_index_range_from_generic_vector"),
                    line: String::from("(start, end)"), 
                    msg: String::from("No bound passed.")
                });
            }
        };

        match v.get(r) {
            Some(ir_v) => return Ok(EFOk { 
                value: ir_v.to_vec(), 
                msg: format!("Cloned object from index {} to {}", s, e)
            }),
            None => Err(EFError {
                function: String::from("get_index_from_generic_vector"),
                line: String::from("v.get(r)"), 
                msg: format!("Could not get range from index {} to {}", s, e)
            })
        }
    }
}

pub mod byte_vector {
    use super::result::{EFOk, EFError};
    use crate::elements::{EFByteRep, EFByteRepBuilder, EFVersion};
    use super::generic_vector::{get_index_from_generic_vector, get_index_range_from_generic_vector};

    pub const BYTE_REP_NONE_OFFSET_ENCODING: usize = 0;

    pub fn get_string_from_byte_vector(v: Vec<u8>) -> Result<EFOk<String>, EFError> {
        match String::from_utf8(v) {
            Ok(s) => Ok(EFOk { value: s, msg: String::from("Created string from byte vector.") }),
            Err(_) => Err(EFError{
                function: String::from("get_string_from_byte_vector"), 
                line: String::from("String::from_utf8(v)"), 
                msg: String::from("Passed in byte vector is not compatible with UTF-8.")
            })
        }
    }

    pub fn get_byte_vector_from_enum_and_string(type_byte: u8, type_str: &String) -> Vec<u8> {
        let mut byte_vec: Vec<u8> = vec![type_byte];
        let mut type_str_vec: Vec<u8> = type_str.clone().into_bytes();
        byte_vec.append(&mut type_str_vec);
        byte_vec
    }

    pub fn get_enum_and_string_from_byte_vector(byte_vec: &Vec<u8>) -> Result<EFOk<(u8, String)>, EFError> {
        let type_byte: u8 = match get_index_from_generic_vector(byte_vec, 0) {
            Ok(index_object) => index_object.value,
            Err(e) => { return Err(e); }
        };

        if byte_vec.len() == 1 {
            Ok(EFOk{
                value: (type_byte, String::from("")), 
                msg: String::from("Returned type byte and empty string.")
            })
        }
        else {
            match get_index_range_from_generic_vector(byte_vec, Some(1), None) {
                Ok(index_range) => match get_string_from_byte_vector(index_range.value) {
                    Ok(s) => Ok(EFOk{
                        value: (type_byte, s.value),
                        msg: String::from("Returned type byte and non-empty string.")
                    }),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        }
    }

    pub fn get_byte_rep_from_builder(brb: &mut EFByteRepBuilder) -> Result<EFOk<EFByteRep>, EFError> {
        // Create a new bytes vector
        let attribute_count: usize = brb.byte_vectors.len() + 2;
        let mut bytes: Vec<u8> = vec![attribute_count as u8];

        // Create a length vector
        let mut lengths: Vec<usize> = vec![brb.version_vector.len(), brb.component_vector.len()];
        let mut byte_lengths: Vec<usize> = brb.byte_vectors.iter().map(|v| v.len()).collect();
        lengths.append(&mut byte_lengths);

        // Use lengths to create offsets
        let mut cur_offset: u8 = brb.byte_vectors.len() as u8;
        for i in 0..brb.byte_vectors.len() {
            match lengths[i] {
                BYTE_REP_NONE_OFFSET_ENCODING => { bytes.push(0u8); },
                _ => { bytes.push(cur_offset); }
            }
            cur_offset = cur_offset + lengths[i] as u8;
        }

        // Add metadata
        bytes.append(&mut brb.version_vector);
        bytes.append(&mut brb.component_vector);

        // Add data
        for i in 0..brb.byte_vectors.len() {
            let mut byte_vector: &mut Vec<u8> = match brb.byte_vectors.get_mut(i) {
                Some(bv) => bv,
                None => {
                    return Err(EFError{
                        function: String::from("get_byte_rep_from_builder"), 
                        line: String::from("brb.byte_vectors.get_mut(i)"), 
                        msg: format!("Got a bad index for byte vector set.")
                    });
                }
            };
            bytes.append(&mut byte_vector);
        }

        // Return byte rep
        Ok(EFOk{
            value: EFByteRep { bytes },
            msg: String::from("Created byte rep.")
        })
    }

    pub fn get_builder_from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<EFByteRepBuilder>, EFError> {
        // Get the attribute count
        let attribute_count: usize = match get_index_from_generic_vector(&byte_rep.bytes, 0) {
            Ok(a) => a.value as usize,
            Err(e) => { return Err(e); }
        };

        // Create empty byte vector set
        let mut byte_vectors: Vec<Vec<u8>> = Vec::new();

        // One loop to grab the offsets
        let mut offsets: Vec<usize> = Vec::new();
        for i in 0..attribute_count {
            offsets.push(byte_rep.bytes[1+i] as usize);
        }

        // Another loop to get all but the last vector
        for i in 0..(attribute_count-1) {
            // Check if the offset is set to the None encoding
            if offsets[i] == BYTE_REP_NONE_OFFSET_ENCODING {
                byte_vectors.push(Vec::new());
            }
            else if let Some(v) = byte_rep.bytes.get(offsets[i]..offsets[i+1]) {
                byte_vectors.push(v.to_vec());
            }
            else {
                return Err(EFError{
                    function: String::from("get_byte_vectors_from_byte_rep"), 
                    line: String::from("byte_rep.bytes.get(offsets[i]..offsets[i+1])"), 
                    msg: format!("Got a bad index for byte rep.")
                });
            }
        }

        // Add the last vector
        if offsets[attribute_count-1] == BYTE_REP_NONE_OFFSET_ENCODING {
            byte_vectors.push(Vec::new());
        }
        else if let Some(v) = byte_rep.bytes.get(offsets[attribute_count-1]..) {
            byte_vectors.push(v.to_vec());
        }
        else {
            return Err(EFError{
                function: String::from("get_byte_vectors_from_byte_rep"), 
                line: String::from("byte_rep.bytes.get(offsets[i]..offsets[i+1])"), 
                msg: format!("Got a bad index for byte rep.")
            });
        }

        // Deconstruct vectors into a builder
        let version_vector: Vec<u8> = byte_vectors.remove(0);
        let component_vector: Vec<u8> = byte_vectors.remove(0);

        Ok(EFOk{
            value: EFByteRepBuilder { byte_vectors, version_vector, component_vector },
            msg: String::from("Created byte vector set.")
        })
    }

    pub fn check_component_and_get_version_from_builder(
        builder: &EFByteRepBuilder, component_str: &str
    ) -> Result<EFOk<EFVersion>, EFError> {
        // Check the component
        let component: String = match get_string_from_byte_vector(builder.component_vector.clone()) {
            Ok(s) => s.value,
            Err(e) => { return Err(e); }
        };
        if !component.eq(component_str) {
            return Err(EFError{
                function: String::from("extract_component_and_version_from_builder"), 
                line: format!("!component.eq(\"{}\")", component_str), 
                msg: format!("Component is not set to {}.", component_str)
            });
        }

        // Get the version
        match builder.version_vector.len() == 3 {
            true => Ok(EFOk{
                value: EFVersion(
                    builder.version_vector[0], 
                    builder.version_vector[1], 
                    builder.version_vector[2]
                ),
                msg: format!("Returned {}'s version.", component_str)
            }),
            false => Err(EFError{
                function: String::from("extract_component_and_version_from_builder"), 
                line: String::from("builder.version_vector.len() == 3"), 
                msg: format!("Could not parse version for {}.", component_str)
            })
        }
    }

    pub fn get_byte_vectors_and_version_from_byte_rep(
        byte_rep: &EFByteRep, component_str: &str
    ) -> Result<EFOk<(Vec<Vec<u8>>, EFVersion)>, EFError> {
        // Get the vectors for each attribute
        let builder: EFByteRepBuilder = match get_builder_from_byte_rep(byte_rep) {
            Ok(b) => b.value,
            Err(e) => { return Err(e); }
        };

        // Check the component and get the version
        let version: EFVersion = match check_component_and_get_version_from_builder(&builder, component_str) {
            Ok(v) => v.value,
            Err(e) => { return Err(e); }
        };

        // Return byte vectors and version directly
        Ok(EFOk{
            value: (builder.byte_vectors, version),
            msg: format!("Returned {}'s attributes and version.", component_str)
        })
    }
}
