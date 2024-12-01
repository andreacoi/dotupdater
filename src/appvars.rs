use std::path::PathBuf;

pub const APP_NAME: &str = "dotupdater";
pub const CONFIG_FILE: &str = "config.toml";
pub const BLUEPRINT_FILE: &str = include_str!("../config.toml.demo");
pub const LOGDIR: &str = "/var/tmp/dotupdater_logs/";
pub const LOGFILE: &str = "dotupdater.log";

fn get_config_dir() -> Option<PathBuf> {
    let config_dir = dirs::config_dir();
    config_dir
}

// function to retrieve log path WITHOUT LOG FILE
pub fn get_complete_log_file_path() -> String {
    let complete_log_file_path: String = format!("{}", LOGDIR);
    complete_log_file_path
}

// function to retrieve config path WITHOUT CONFIG FILE
pub fn get_complete_config_file_path() -> String {
    let complete_config_file_path: String = format!(
        "{}/{}",
        get_config_dir().unwrap().to_str().unwrap().to_owned(),
        APP_NAME
    );
    complete_config_file_path
}

pub fn prepend_dir(dir: String) -> String {
    let base_path: String = get_config_dir().unwrap().to_str().unwrap().to_owned();
    let full_dir: String = format!("{}/{}", base_path, dir);
    full_dir
}
