use std::collections::HashMap;
use serde_json::{Value as JSONValue};
use crate::utils::json::load_json_from_file;

#[derive(Debug)]
pub struct StartingConfig {
    pub arg_file: String,
    pub app_data_folder: String,
    pub create_folder_if_no_parent: String,
    pub entity_hash: String
}

fn check_arg_file(starting_config: &mut StartingConfig) {
    // Get JSON contents from arg file
    let arg_file_json: JSONValue = match load_json_from_file(&starting_config.arg_file) {
        Ok(j) => j.value,
        Err(e) => { panic!("{}", &e.to_string().as_str()); }
    };
    
    // Try to fill in any empty attributes
    let get_json_value_or_self = | arg_key: &str, cur_arg_value: &str | -> String {
        // Only replace if the current arg value hasn't been set yet
        match cur_arg_value.is_empty() {
            false => cur_arg_value.to_string(),

            // Overwrite only if there is an existing arg value for the key
            true => match arg_file_json.get(arg_key) {
                Some(arg_value) => arg_value.to_string(),
                None => cur_arg_value.to_string()
            }
        }
    };

    starting_config.app_data_folder = get_json_value_or_self(
        "app_data_folder", starting_config.app_data_folder.as_str()
    );
    starting_config.create_folder_if_no_parent = get_json_value_or_self(
        "create_folder_if_no_parent", starting_config.create_folder_if_no_parent.as_str()
    );
    starting_config.entity_hash = get_json_value_or_self(
        "entity_hash", starting_config.entity_hash.as_str()
    );
}

pub fn parse_cli_arguments(args: Vec<String>) -> StartingConfig {
    // Create empty map and starting index
    let mut arg_map: HashMap<String, String> = HashMap::new();
    let mut arg_index: usize = 1;

    // Process arguments that are structured like "-key value" or "key=value"
    while arg_index < args.len() {
        let cur_arg: &String = args.get(arg_index).expect("Expected a CLI argument key.");
        if cur_arg.starts_with("-") {
            let cur_arg_value: &String = args.get(arg_index+1).expect("Expected a CLI argument value.");
            arg_map.insert(cur_arg.clone().replace("-", ""), cur_arg_value.clone());
            arg_index = arg_index + 2;
        }
        else if cur_arg.contains("=") {
            let cur_arg_split: Vec<&str> = cur_arg.split("=").collect();
            arg_map.insert(String::from(cur_arg_split[0]), String::from(cur_arg_split[1]));
            arg_index = arg_index + 1;
        }
    }

    // Fill in global config with everything
    let get_map_value_or_empty = | arg_key: &str | -> String {
        String::from(arg_map.get(&String::from(arg_key)).unwrap_or(&String::from("")))
    };

    let mut starting_config: StartingConfig = StartingConfig { 
        arg_file: get_map_value_or_empty("arg_file"), 
        app_data_folder: get_map_value_or_empty("app_data_folder"),
        create_folder_if_no_parent: get_map_value_or_empty("create_folder_if_no_parent"),
        entity_hash: get_map_value_or_empty("entity_hash")
    };

    if !starting_config.arg_file.is_empty() {
        check_arg_file(&mut starting_config);
    }

    starting_config
}
