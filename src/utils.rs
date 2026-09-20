pub mod element_types {
    // Unsigned integers
    pub const EFUSIZE_TYPE: &'static str = "usize";
    pub const EFU8_TYPE: &'static str = "u8";
    pub const EFU16_TYPE: &'static str = "u16";
    pub const EFU32_TYPE: &'static str = "u32";
    pub const EFU64_TYPE: &'static str = "u64";
    pub const EFU128_TYPE: &'static str = "u128";

    // Signed integers
    pub const EFISIZE_TYPE: &'static str = "isize";
    pub const EFI8_TYPE: &'static str = "i8";
    pub const EFI16_TYPE: &'static str = "i16";
    pub const EFI32_TYPE: &'static str = "i32";
    pub const EFI64_TYPE: &'static str = "i64";
    pub const EFI128_TYPE: &'static str = "i128";

    // Signed floats
    pub const EFF32_TYPE: &'static str = "f32";
    pub const EFF64_TYPE: &'static str = "f64";

    // Other primitives
    pub const EFBOOL_TYPE: &'static str = "bool";
    pub const EFCHAR_TYPE: &'static str = "char";

    // Common components
    pub const EFSTRING_TYPE: &'static str = "string";

    // Core components
    pub const EFIDENTITY_TYPE: &'static str = "identity";
    pub const EFROLE_TYPE: &'static str = "role";
    pub const EFROLEVECTOR_TYPE: &'static str = "role_vector";
    pub const EFSECRET_TYPE: &'static str = "secret";
}

pub mod element_versions {
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
    use crate::elements::EFVersion;

    pub const EMPTY_VERSION: EFVersion = EFVersion(0, 0, 0);
    pub const EMPTY_STR_SLICE: &'static str = "";
    pub const DEFAULT_LOG_DELIMITER: char = '\n';
}

pub mod result {
    use super::constants::DEFAULT_LOG_DELIMITER;

    #[derive(Debug, Clone)]
    pub struct EFReturnEvent {
        logs: Vec<String>
    }

    impl EFReturnEvent {
        pub fn new() -> EFReturnEvent {
            EFReturnEvent { logs: Vec::new() }
        }

        pub fn add_log(&mut self, new_log: String) {
            self.logs.push(new_log);
        }

        pub fn add_func_info_log(&mut self, func_name: &str, info: &str) {
            self.logs.push(format!("{}: {}", func_name, info));
        }

        pub fn get_delimited_logs(&self, delimiter: char) -> String {
            if self.logs.is_empty() {
                String::new()
            }
            else {
                let mut delimited_logs: String = String::new();

                for log in self.logs.iter() {
                    delimited_logs.push_str(log.as_str());
                    delimited_logs.push(delimiter);
                }

                delimited_logs
            }
        }

        pub fn get_default_delimited_logs(&self) -> String {
            self.get_delimited_logs(DEFAULT_LOG_DELIMITER)
        }

        pub fn pop_earliest_log(&mut self) -> Option<String> {
            if self.logs.is_empty() {
                None
            }
            else {
                Some(self.logs.remove(0))
            }
        }

        pub fn new_with_log(new_log: String) -> EFReturnEvent {
            let mut new_event: EFReturnEvent = EFReturnEvent::new();
            new_event.add_log(new_log);
            new_event
        }

        pub fn new_with_func_info_log(func_name: &str, info: &str) -> EFReturnEvent {
            let mut new_event: EFReturnEvent = EFReturnEvent::new();
            new_event.add_func_info_log(func_name, info);
            new_event
        }

        pub fn transfer_event(&mut self, event: EFReturnEvent) {
            for log in event.logs.into_iter() {
                self.logs.push(log);
            }
        }

        pub fn strip_event_from_return<T>(&mut self, ret: EFReturn<T>) -> T {
            let (value, event) = (ret.value, ret.event);
            self.transfer_event(event);
            value
        }
    }

    #[derive(Debug, Clone)]
    pub struct EFReturn<T> {
        pub value: T,
        pub event: EFReturnEvent
    }

    #[derive(Debug, Clone)]
    pub enum EFReturnEnum<T> {
        Value(T),
        Return(EFReturn<T>)
    }

    impl<T> EFReturn<T> {
        pub fn new(new_value: T) -> EFReturn<T> {
            EFReturn { value: new_value, event: EFReturnEvent::new() }
        }

        pub fn compose(new_value: T, new_event: EFReturnEvent) -> EFReturn<T> {
            EFReturn { value: new_value, event: new_event }
        }

        pub fn from_enum(ret_enum: EFReturnEnum<T>) -> EFReturn<T> {
            match ret_enum {
                EFReturnEnum::Value(v) => EFReturn::new(v),
                EFReturnEnum::Return(r) => r
            }
        }

        pub fn run_mutator(&mut self, mut mutator_func: impl FnMut(&mut EFReturn<T>)) {
            mutator_func(self);
        }

        pub fn run_returner(&self, return_func: impl Fn(&EFReturn<T>) -> EFReturn<T>) -> EFReturn<T> {
            return_func(&self)
        }

        pub fn run_creator(new_value: T, create_func: impl Fn(T) -> EFReturn<T>) -> EFReturn<T> {
            create_func(new_value)
        }

        pub fn run_creator_with_enum(
            ret_enum: EFReturnEnum<T>,
            enum_create_func: impl Fn(EFReturnEnum<T>) -> EFReturn<T>
        ) -> EFReturn<T> {
            enum_create_func(ret_enum)
        }

        pub fn new_with_log(new_value: T, new_log: String) -> EFReturn<T> {
            let mut new_return: EFReturn<T> = EFReturn::new(new_value);
            new_return.event.add_log(new_log);
            new_return
        }

        pub fn new_with_func_info_log(new_value: T, func_name: &str, info: &str) -> EFReturn<T> {
            let mut new_return: EFReturn<T> = EFReturn::new(new_value);
            new_return.event.add_func_info_log(func_name, info);
            new_return
        }

        pub fn from_enum_with_log(ret_enum: EFReturnEnum<T>, new_log: String) -> EFReturn<T> {
            let mut new_return: EFReturn<T> = EFReturn::from_enum(ret_enum);
            new_return.event.add_log(new_log);
            new_return
        }

        pub fn from_enum_with_func_info_log(
            ret_enum: EFReturnEnum<T>, 
            func_name: &str, 
            info: &str
        ) -> EFReturn<T> {
            let mut new_return: EFReturn<T> = EFReturn::from_enum(ret_enum);
            new_return.event.add_func_info_log(func_name, info);
            new_return
        }

        pub fn decompose(&self) -> (T, EFReturnEvent) {
            (self.value, self.event)
        }
    }

    pub type EFValueResult<T> = Result<T, EFReturnEvent>;

    pub type EFResult<T> = Result<EFReturn<T>, EFReturnEvent>;
}

pub mod general {
    use super::result::{EFReturnEvent, EFValueResult};
    use sha2::{Digest, Sha256, Sha512};

    pub fn get_hash(string_vec: Vec<&String>, hash: &String) -> EFValueResult<String> {
        let hash_bytes: Vec<u8> = match hash.to_lowercase().as_str() {
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
                return Err(EFReturnEvent::new_with_func_info_log(
                    "get_hash", 
                    "An incorrect value was used for hash key."
                ));
            }
        };

        match String::from_utf8(hash_bytes) {
            Ok(h) => Ok(h),
            Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                "get_hash", 
                "get_hash failed to make a string from a vector."
            ))
        }
    }
}

pub mod json {
    use super::result::{EFValueResult, EFReturnEvent};
    use std::{io::Read, path::Path};
    use std::fs::File;
    use serde_json::{Value as JSONValue, from_str as json_from_str};

    pub fn load_json_from_file(file_str: &str) -> EFValueResult<JSONValue> {
        // Create a path
        let file_path: &Path = Path::new(file_str);

        // Open the file
        let mut file_obj: File = match File::open(file_path) {
            Err(_) => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "load_json_from_file", 
                    format!("Could not open file at {}.", file_str).as_str()
                ));
            },
            Ok(f) => f
        };

        // Read the file to string
        let mut file_str: String = String::new();
        if file_obj.read_to_string(&mut file_str).is_err() {
            return Err(EFReturnEvent::new_with_func_info_log(
                "load_json_from_file", 
                format!("Could not read file at {}.", file_str).as_str()
            ));
        }

        // Parse JSON from string
        let file_json: JSONValue = match json_from_str(file_str.as_str()) {
            Err(_) => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "load_json_from_file", 
                    format!("Could not parse JSON from {}.", file_str).as_str()
                ));
            },
            Ok(j) => j
        };

        // Return the JSON value
        Ok(file_json)
    }
}

pub mod membership {
    use std::collections::HashSet;
    use std::hash::Hash;

    pub fn union_of<T: Clone + Hash + Eq>(first_vector: &Vec<T>, second_vector: &Vec<T>) -> Vec<T> {
        let mut union_set: HashSet<T> = HashSet::new();

        for item in first_vector {
            union_set.insert(item.clone());
        }

        for item in second_vector {
            union_set.insert(item.clone());
        }

        union_set.into_iter().collect()
    }

    pub fn intersection_of<T: Clone + Hash + Eq>(first_vector: &Vec<T>, second_vector: &Vec<T>) -> Vec<T> {
        let mut first_set: HashSet<T> = HashSet::new();
        let mut intersection_vec: Vec<T> = Vec::new();

        for item in first_vector {
            first_set.insert(item.clone());
        }

        for item in second_vector {
            if first_set.contains(item) {
                intersection_vec.push(item.clone());
            }
        }

        intersection_vec
    }

    pub fn not_intersection_of<T: Clone + Hash + Eq>(first_vector: &Vec<T>, second_vector: &Vec<T>) -> Vec<T> {
        let mut not_intersection_set: HashSet<T> = HashSet::new();

        for item in first_vector {
            not_intersection_set.insert(item.clone());
        }

        for item in second_vector {
            if not_intersection_set.contains(item) {
                not_intersection_set.remove(item);
            }
            else {
                not_intersection_set.insert(item.clone());
            }
        }

        not_intersection_set.into_iter().collect()
    }

    pub fn only_in_first<T: Clone + Hash + Eq>(first_vector: &Vec<T>, second_vector: &Vec<T>) -> Vec<T> {
        let mut only_first_set: HashSet<T> = HashSet::new();

        for item in first_vector {
            only_first_set.insert(item.clone());
        }

        for item in second_vector {
            if only_first_set.contains(item) {
                only_first_set.remove(item);
            }
        }

        only_first_set.into_iter().collect()
    }

    pub fn is_subset_of_first<T: Clone + Hash + Eq>(first_vector: &Vec<T>, second_vector: &Vec<T>) -> bool {
        let mut first_set: HashSet<T> = HashSet::new();

        for item in first_vector {
            first_set.insert(item.clone());
        }

        for item in second_vector {
            if !first_set.contains(item) {
                return false;
            }
        }

        true
    }
}

pub mod os {
    use super::result::{EFValueResult, EFReturnEvent};
    use std::env::consts::OS;
    use std::env::var as env_var;
    use std::path::Path;
    use std::fs::{create_dir, create_dir_all};

    pub fn get_os_default_folder() -> EFValueResult<String> {
        match OS {
            "windows" => {
                match env_var("APPDATA") {
                    Ok(v) => Ok(v),
                    Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                        "get_os_default_folder", 
                        "Could not find the default application folder for Windows. Please set APPDATA."
                    ))
                }
            },
            "linux" | "macos" => {
                match env_var("HOME") {
                    Ok(v) => Ok(v),
                    Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                        "get_os_default_folder", 
                        "Could not find the user's home directory for Linux\\Mac OS. Please set HOME."
                    ))
                }
            },
            _ => panic!("Other operating systems aren't supported.")
        }
    }

    pub fn create_folder(path_str: &String, cfinp: &String) -> EFValueResult<()> {
        let path: &Path = Path::new(path_str.as_str());
        match path.is_dir() {
            false => {
                match cfinp.to_lowercase().as_str() {
                    "true" | "yes" => {
                        match create_dir_all(path) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                                "create_folder", 
                                format!("Unable to create all directories for {}.", path_str).as_str()
                            ))
                        }
                    },
                    "false" | "no" | "" => {
                        match create_dir(path) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                                "create_folder", 
                                format!("Unable to create directory for {}.", path_str).as_str()
                            ))
                        }
                    },
                    _ => Err(EFReturnEvent::new_with_func_info_log(
                        "create_folder", 
                        "An incorrect value was used for create_folder_if_no_parent key."
                    ))
                }
            }
            true => Err(EFReturnEvent::new_with_func_info_log(
                "create_folder", 
                format!("Directory for {} already exists.", path_str).as_str()
            ))
        }
    }
}

pub mod vector {
    use super::result::{EFValueResult, EFReturnEvent};

    pub fn get_generic_vec_obj<T: Clone>(generic_vec: &Vec<T>, obj_index: usize) -> EFValueResult<T> {
        match generic_vec.get(obj_index) {
            Some(obj) => Ok(obj.clone()),
            None => Err(EFReturnEvent::new_with_func_info_log(
                "get_generic_vec_obj", 
                format!("Could not get object at index {}.", obj_index).as_str()
            ))
        }
    }

    pub fn get_multiple_generic_vec_obj<T: Clone>(
        generic_vec: &Vec<T>, 
        start_op: Option<usize>, 
        end_op: Option<usize>
    ) -> EFValueResult<Vec<T>> {
        let (index_range, start_index, end_index) = match (start_op, end_op) {
            // Get only between the bounds
            (Some(s), Some(e)) => (s..e, s, e),

            // Get the rest of index after start
            (Some(s), None) => (s..generic_vec.len(), s, generic_vec.len()),

            // Get beginning of index up to end
            (None, Some(e)) => (0..e, 0, e),

            // Why would you do this
            (None, None) => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "get_multiple_generic_vec_obj", 
                    "No bound passed."
                ));
            }
        };

        match generic_vec.get(index_range) {
            Some(obj_slice) => Ok(obj_slice.to_vec()),
            None => return Err(EFReturnEvent::new_with_func_info_log(
                "get_multiple_generic_vec_obj", 
                format!("Could not get range from index {} to {}.", start_index, end_index).as_str()
            ))
        }
    }

    pub fn get_str_slice_from_vec_u8(vec_u8: &Vec<u8>) -> EFValueResult<&str> {
        match str::from_utf8(vec_u8.as_slice()) {
            Ok(s) => Ok(s),
            Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                "get_str_slice_from_vec_u8", 
                "Passed in byte vector is not compatible with UTF-8."
            ))
        }
    }
}

pub mod hashmap {
    use std::collections::HashMap;

    pub fn get_keys_vec<K: Clone, V>(generic_hashmap: &HashMap<K, V>) -> Vec<K> {
        generic_hashmap.keys().cloned().collect()
    }

    pub fn get_values_vec<K, V: Clone>(generic_hashmap: &HashMap<K, V>) -> Vec<V> {
        generic_hashmap.values().cloned().collect()
    }
}
