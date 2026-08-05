// Top level modules
pub mod utils;
pub mod elements;
pub mod config;
pub mod loader;
// pub mod auth;

// Scope section
use std::env;
use config::{StartingConfig, parse_cli_arguments};
// use loader::ProgramState;
// use crate::auth::authentication;
// use crate::utils::GlobalInfo;

fn main() {
    // Argument collecting and parsing
    let args: Vec<String> = env::args().collect();
    let starting_config: StartingConfig = parse_cli_arguments(args);
    dbg!(&starting_config);

    // Loading program state
    // let program_state: ProgramState;

    // let global_info: GlobalInfo = GlobalInfo::new();
    // dbg!(global_info);

    // Authentication
    //authentication(&starting_config);

    // Working with systems

    // Loading Plugins

    // Starting CLI
}
