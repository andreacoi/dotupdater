#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub repositories: Vec<RepositoryConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RepositoryConfig {
    pub path: String,
    pub branch: String,
}
use crate::appvars::{self, get_complete_config_file_path, CONFIG_FILE};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
pub fn create_base_config_path() -> Result<String, String> {
    // create config folder --> return Ok() if the folder can be created
    let complete_config_file_path = get_complete_config_file_path();
    // if the config folder does not exist create it
    if !Path::new(&complete_config_file_path).exists() {
        fs::create_dir(&complete_config_file_path)
            .map_err(|err| format!("Config folder can't be created. Reason: {}", err))?;
        let message: String = String::from("Config folder created successfully");
        return Ok(message);
    } else {
        let message: String = String::from("Config folder is already in the right place.");
        return Err(message);
    }
}

pub fn create_blueprint_config_file() -> Result<String, String> {
    let config_folder_path = get_complete_config_file_path();
    let config_file_path: String = format!("{}/{}", config_folder_path, CONFIG_FILE);

    if !Path::new(&config_folder_path).exists() {
        let _ = create_base_config_path();
        let config_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(config_file_path)
            .map_err(|e| e.to_string())?;
        writeln!(&config_file, "{}", appvars::BLUEPRINT_FILE).map_err(|e| e.to_string())?;
        let message: String = String::from("config.toml file created correctly");
        return Ok(message);
    } else {
        // if the folder exists verify that the config file is already in that folder.
        // config file path = complete_file_path + filename
        if Path::new(&config_file_path).exists() {
            return Ok(String::from("Config file already created."));
        } else {
            println!("{}", &config_file_path);
            let config_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&config_file_path)
                .map_err(|e| e.to_string())?;
            writeln!(&config_file, "{}", appvars::BLUEPRINT_FILE).map_err(|e| e.to_string())?;
            let message: String = String::from("config.toml file created correctly");
            return Ok(message);
        }
    }
}
