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

pub mod versions {
    use crate::elements::EFVersion;

    pub fn get_efidentity_version() -> EFVersion {
        EFVersion(0, 0, 1)
    }
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

pub mod std {
    use super::result::{EFOk, EFError};
    use sha2::{Digest, Sha256, Sha512};
    use crate::elements::EFByteRep;

    pub const BYTE_REP_NONE_OFFSET_ENCODING: usize = 0;

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

    pub fn get_byte_vector_from_enum_and_string(type_byte: u8, type_str: &String) -> Vec<u8> {
        let mut byte_vec: Vec<u8> = vec![type_byte];
        let mut type_str_vec: Vec<u8> = type_str.clone().into_bytes();
        byte_vec.append(&mut type_str_vec);
        byte_vec
    }

    pub fn get_enum_and_string_from_byte_vector(byte_vec: &Vec<u8>) -> Result<EFOk<(u8, String)>, EFError> {
        let type_byte: u8 = match byte_vec.get(0) {
            Some(tb) => tb.clone(),
            None => {
                return Err(EFError{ 
                    function: String::from("get_enum_and_string_from_byte_vec"), 
                    line: String::from("match byte_vec.get(0)"), 
                    msg: String::from("Could not get type byte.")
                })
            }
        };

        if byte_vec.len() == 1 {
            Ok(EFOk{
                value: (type_byte, String::from("")), 
                msg: String::from("Returned type byte and empty string.")
            })
        }
        else {
            match byte_vec.get(1..) {
                Some(s_arr) => match String::from_utf8(s_arr.to_vec()) {
                    Ok(s) => Ok(EFOk{
                        value: (type_byte, s),
                        msg: String::from("Returned type byte and non-empty string.")
                    }),
                    Err(_) => Err(EFError{
                        function: String::from("get_enum_and_string_from_byte_vec"), 
                        line: String::from("String::from_utf8(s_arr.to_vec())"), 
                        msg: String::from("Could not parse string.")
                    })
                },
                None => Err(EFError{
                    function: String::from("get_enum_and_string_from_byte_vec"), 
                    line: String::from("match byte_vec.get(1..)"), 
                    msg: String::from("Could not get string.")
                })
            }
        }
    }

    pub fn get_byte_rep_from_byte_vectors(byte_vectors: &mut Vec<Vec<u8>>) -> Result<EFOk<EFByteRep>, EFError> {
        // Create a length vector
        let lengths: Vec<usize> = byte_vectors.iter().map(|v| v.len()).collect();

        // Create a new bytes vector
        let mut bytes: Vec<u8> = vec![byte_vectors.len() as u8];

        // Add offsets
        let mut cur_offset: u8 = byte_vectors.len() as u8;
        for i in 0..byte_vectors.len() {
            match lengths[i] {
                BYTE_REP_NONE_OFFSET_ENCODING => { bytes.push(0u8); },
                _ => { bytes.push(cur_offset); }
            }
            cur_offset = cur_offset + lengths[i] as u8;
        }

        // Add data
        for i in 0..byte_vectors.len() {
            let mut byte_vector: &mut Vec<u8> = match byte_vectors.get_mut(i) {
                Some(bv) => bv,
                None => {
                    return Err(EFError{
                        function: String::from("get_byte_rep_from_byte_vectors"), 
                        line: String::from("byte_vectors.get_mut(i)"), 
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

    pub fn get_byte_vectors_from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Vec<Vec<u8>>>, EFError> {
        // Get the attribute count
        let attribute_count: usize = match byte_rep.bytes.get(0) {
            Some(a) => a.clone() as usize,
            None => {
                return Err(EFError{
                    function: String::from("get_byte_vectors_from_byte_rep"), 
                    line: String::from("byte_rep.bytes.get(0)"), 
                    msg: format!("Can not get attribute count from byte rep.")
                });
            }
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

        // Return
        Ok(EFOk{ value: byte_vectors, msg: String::from("Created byte vector set.") })
    }
}
