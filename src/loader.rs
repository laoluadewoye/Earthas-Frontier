use crate::config::StartingConfig;
use crate::utils::os::{get_os_default_folder, create_folder};
use crate::elements::std::EFGlobalState;

pub fn load_program_state(starting_config: &StartingConfig) -> EFGlobalState {
    // Confirm the application data folder
    let app_data_folder: String = match starting_config.app_data_folder.is_empty() {
        false => starting_config.app_data_folder.clone(),
        true => {
            match get_os_default_folder() {
                Ok(v) => v.value,
                Err(e) => { panic!("{}", &e.to_string().as_str()); }
            }
        }
    };

    // BEFORE UNCOMMONTING THE BELOW CODE, ADD LOGIC TO CREATE A FOLDER SPECIFICALLY FOR THE PROGRAM, LIKE HOW VIVALDI HAS IT'S OWN FOLDER

    // Check if there is an existing folder at the desired location, and try to create one if not
    // let create_folder_result = match create_folder(
    //     &app_data_folder, 
    //     &starting_config.create_folder_if_no_parent
    // ) {
    //     Ok(s) => s,
    //     Err(e) => { panic!("{}", &e.to_string().as_str()); }
    // };
    // let adf_path: &Path = Path::new(app_data_folder.as_str());

    // Start loading JSON files from the app directory, or creating new fresh data
}
