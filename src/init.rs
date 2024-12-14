// fn precheck?
// if config == true && log == true - split if into precheck
// not initialize_config
// not initialize_log
// else
// initialize element that aren't initialized
use crate::config::{create_base_config_path, create_blueprint_config_file};
use crate::logger::{create_base_logfiles_path, create_log_file, logevent, EventType};
// this function create all the enviroment.
pub fn initialize() {
    // create log folder
    let _ = create_base_logfiles_path();
    // create log file
    let _ = create_log_file();

    // log starting app
    let _ = logevent(
        String::from("Starting app..."),
        EventType::I(String::from("[I]")),
    );

    match create_base_config_path() {
        Ok(message) => {
            let _ = logevent(message, EventType::I(String::from("[I]")));
        }
        Err(message) => {
            let _ = logevent(message, EventType::E(String::from("[E]")));
        }
    }

    match create_blueprint_config_file() {
        Ok(message) => {
            let _ = logevent(message, EventType::I(String::from("[I]")));
        }
        Err(message) => {
            let _ = logevent(message, EventType::E(String::from("[E]")));
        }
    }

    // get config_dir by enviroment
    // !!!! todo: manage existence of config folder and config file
    // tip: if the folder does not exist then even the file can't exist (logically)
    // fn create_config_folder -> Ok(folder_name)
    // fn create_config_file -> Ok(())
    // get program folder - dotupdater in this case - e.g. /home/johndoe/.config/dotupdater
}
