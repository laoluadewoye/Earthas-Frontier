pub mod component_types {
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
    pub const EFROLE_STR: &'static str = "role";
    pub const EFROLEVECTOR_STR: &'static str = "role_vector";
    pub const EFSECRET_STR: &'static str = "secret";
}

pub mod component_versions {
    use crate::elements::EFVersion;
    
    // Unsigned integers
    pub const EFUSIZE_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFU8_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFU16_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFU32_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFU64_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFU128_VERSION: EFVersion = EFVersion(0, 1, 0);

    // Signed integers
    pub const EFISIZE_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFI8_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFI16_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFI32_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFI64_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFI128_VERSION: EFVersion = EFVersion(0, 1, 0);

    // Signed floats
    pub const EFF32_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFF64_VERSION: EFVersion = EFVersion(0, 1, 0);

    // Other primitives
    pub const EFBOOL_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFCHAR_VERSION: EFVersion = EFVersion(0, 1, 0);

    // Common components
    pub const EFSTRING_VERSION: EFVersion = EFVersion(0, 1, 0);

    // Core components
    pub const EFIDENTITY_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFROLE_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFROLEVECTOR_VERSION: EFVersion = EFVersion(0, 1, 0);
    pub const EFSECRET_VERSION: EFVersion = EFVersion(0, 1, 0);
}

pub mod constants {
    pub const EMPTY_STR_SLICE: &'static str = "";
}

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

pub mod vector {
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

    pub fn get_string_from_byte_vector(v: &Vec<u8>) -> Result<EFOk<&str>, EFError> {
        match str::from_utf8(v.as_slice()) {
            Ok(s) => Ok(EFOk{ value: s, msg: String::from("Created string from byte vector.") }),
            Err(_) => Err(EFError{
                function: String::from("get_string_from_byte_vector"), 
                line: String::from("str::from_utf8(v.as_slice())"), 
                msg: String::from("Passed in byte vector is not compatible with UTF-8.")
            })
        }
    }
}
