use crate::appvars::{self, LOGDIR, LOGFILE};
use crate::ghutils::get_config_list;
use chrono::prelude::*;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

// Returns a string with a date to be placed in log file.
fn get_task_datetime() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted_date
}

// function to create log folder if it's not exist
pub fn create_base_logfiles_path() -> Result<(), String> {
    // create config folder --> return Ok() if the folder can be created
    let complete_log_file_path = appvars::get_complete_log_file_path();
    // if the config folder does not exist create it
    if !Path::new(&complete_log_file_path).exists() {
        fs::create_dir(&complete_log_file_path)
            .map_err(|err| format!("Log folder can't be created. Reason: {}", err))?;
    }
    Ok(())
}
// function to create log file if it's not exist.
pub fn create_log_file() -> Result<(), String> {
    let file_path = format!(
        "{}/{}",
        appvars::get_complete_log_file_path(),
        appvars::LOGFILE
    );
    if !Path::new(&file_path).exists() {
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&file_path);
    }
    Ok(())
}

fn get_logfile_path() -> String {
    let logfile_path: String = format!("{}{}", LOGDIR, LOGFILE);
    logfile_path
}

// create an enum with event types.
// [I] [W] [!] [E]
// [I] Info - simple Info
// [W] Warning
// [II] Notice
// [E] Error
pub enum EventType {
    I(String),
    W(String),
    N(String),
    E(String),
}
// !!!! todo: manage existence of log folder and log file
// tip: if the folder does not exist then even the file can't exist (logically)
// fn create_log_folder -> Ok(folder_name)
// fn create_log_file -> Ok(())
// function to log all events in the lifecycle of the app.

pub fn logevent(message: String, event_type: EventType) -> std::io::Result<()> {
    // retrieves datetime from get_task_datetime
    let datetime_now = get_task_datetime();
    let event_prefix = match event_type {
        EventType::W(_) => String::from("[W]"),
        EventType::I(_) => String::from("[I]"),
        EventType::N(_) => String::from("[!!]"),
        EventType::E(_) => String::from("[E]"),
    };
    let log_message = format!("{} - {} - {}", datetime_now, event_prefix, message);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_logfile_path())?;
    // function to write to the file
    writeln!(file, "{}", log_message)?;
    Ok(())
}
// log all events regarding questions like folders, files...
// log fetch
// log pull
