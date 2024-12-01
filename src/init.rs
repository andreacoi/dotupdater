// fn precheck?
// if config == true && log == true - split if into precheck
// not initialize_config
// not initialize_log
// else
// initialize element that aren't initialized

// this function create all the enviroment.
pub fn initialize() {
    // create log folder
    logger::create_base_logfiles_path();
    // create log file
    logger::create_log_file();
    let logfile_path = format!(
        "{}{}",
        appvars::get_complete_log_file_path(),
        appvars::LOGFILE
    );

    // log starting app
    logevent(
        &logfile_path,
        String::from("Starting app..."),
        logger::EventType::I(String::from("[I]")),
    );

    match create_base_config_path() {
        Ok(message) => {
            logevent(
                &logfile_path,
                message,
                logger::EventType::I(String::from("[I]")),
            );
        }
        Err(message) => {
            logevent(
                &logfile_path,
                message,
                logger::EventType::E(String::from("[E]")),
            );
        }
    }

    match create_blueprint_config_file() {
        Ok(message) => {
            logevent(
                &logfile_path,
                message,
                logger::EventType::I(String::from("[I]")),
            );
        }
        Err(message) => {
            logevent(
                &logfile_path,
                message,
                logger::EventType::E(String::from("[E]")),
            );
        }
    }

    // get config_dir by enviroment
    // !!!! todo: manage existence of config folder and config file
    // tip: if the folder does not exist then even the file can't exist (logically)
    // fn create_config_folder -> Ok(folder_name)
    // fn create_config_file -> Ok(())
    // get program folder - dotupdater in this case - e.g. /home/johndoe/.config/dotupdater
}
