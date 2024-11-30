#[derive(Debug, serde::Deserialize)]
pub struct Config {
    repositories: Vec<RepositoryConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RepositoryConfig {
    path: String,
    branch: String,
}

pub mod appvars {
    use std::path::{Path, PathBuf};

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
}

pub mod init {
    use std::fs::{self, OpenOptions};
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    use chrono::format;
    use toml::ser::Error;

    use crate::appvars;
    use crate::logger::{self, logevent};

    fn create_base_config_path() -> Result<(), String> {
        // create config folder --> return Ok() if the folder can be created
        let complete_config_file_path = appvars::get_complete_config_file_path();
        // if the config folder does not exist create it
        if !Path::new(&complete_config_file_path).exists() {
            fs::create_dir(&complete_config_file_path)
                .map_err(|err| format!("Config folder can't be created. Reason: {}", err))?;
            println!("Config folder created successfully!");
        } else {
            println!("Config folder already exists");
        }
        Ok(())
    }

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
        // get config_dir by enviroment
        // !!!! todo: manage existence of config folder and config file
        // tip: if the folder does not exist then even the file can't exist (logically)
        // fn create_config_folder -> Ok(folder_name)
        // fn create_config_file -> Ok(())
        // get program folder - dotupdater in this case - e.g. /home/johndoe/.config/dotupdater
    }
}

pub mod logger {
    use crate::appvars;
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
            println!("Log folder created successfully!");
        } else {
            println!("Log folder already exists");
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
            let mut log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_path);
        } else {
            println!("Log file already exists");
        }
        Ok(())
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

    pub fn logevent(
        file_path: &String,
        message: String,
        event_type: EventType,
    ) -> std::io::Result<()> {
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
            .open(file_path)?;
        // function to write to the file
        writeln!(file, "{}", log_message)?;
        Ok(())
    }
    // log all events regarding questions like folders, files...
    // log fetch
    // log pull
}

pub mod utils {
    // check internet connection
    use std::net::TcpStream;
    use std::thread::sleep;
    use std::time::Duration;
    fn check_internet_connection() -> bool {
        TcpStream::connect("8.8.8.8:53").is_ok()
    }

    // function to implement an infinite cycle if the computer is connected. while the computer is not
    // connected sleep for 3 second then re-check the connection.
    pub fn wait_for_connection() {
        while !check_internet_connection() {
            sleep(Duration::from_secs(3));
        }
    }
}

pub mod github_utils {
    use git2::build::CheckoutBuilder;
    use git2::{Error, FetchOptions, RemoteCallbacks, Repository};

    // Function to be used as a callback for fetch_options. It is used for authentication and relies
    // on the system's SSH agent (provided it is enabled). The remote.fetch() function, in fact,
    // uses options for authentication when accessing private repositories as its second argument.
    // These options can be represented by callback functions (declared in Rust with ||).
    // Finally, I use "git" as the username and the "public --> private" key pair for authentication.

    fn create_credentials_callback<'a>() -> RemoteCallbacks<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username, _allowed| git2::Cred::ssh_key_from_agent("git"));
        callbacks
    }

    // I split the functions into fetch and pull. If fetch is false, the program terminates;
    // otherwise, proceed with the pull. If you proceed with the pull, write the output to the log, otherwise don't.

    // The fetch function is responsible only for checking if there are updates on the remote repository.
    // If updates exist, perform the pull in the pull function, otherwise, do nothing.
    // I use the git2 library, which, starting from a "local machine" path, "opens" the repository
    // (by reading the .git folder) via repo_path, retrieves the origin, and performs the fetch
    // on the branch specified in &branch.

    pub fn fetch(repo_path: &str, branch: &str) -> Result<(), Error> {
        let repo = Repository::open(repo_path)?;
        let mut remote = repo.find_remote("origin")?;

        // create a FetchOption object in order to perform auth
        let mut fetch_options = FetchOptions::new();
        // prepare fetch to use callbacks
        fetch_options.remote_callbacks(create_credentials_callback());

        remote.fetch(&[branch], Some(&mut fetch_options), None)?;
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.is_up_to_date() {
            Ok(())
        } else {
            Err(Error::from_str("Repository has updates available!"))
        }
    }

    // pull function - useful only when fetch returns an error
    pub fn pull(repo_path: &str, branch: &str) -> Result<(), Error> {
        let repo = Repository::open(repo_path)?;
        let mut remote = repo.find_remote("origin")?;
        // create a FetchOption object in order to perform auth
        let mut fetch_options = FetchOptions::new();
        // prepare fetch to use callbacks
        fetch_options.remote_callbacks(create_credentials_callback());
        // execute fetch
        remote.fetch(&[branch], Some(&mut fetch_options), None)?;

        // check head and commit
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.is_up_to_date() {
            return Ok(());
        }
        if analysis.is_fast_forward() {
            let branch = repo.find_branch(branch, git2::BranchType::Local)?;
            let mut branch_ref = branch.into_reference();
            branch_ref.set_target(fetch_commit.id(), "Fast Forward")?;
            // force the checkout of a new version
            let mut checkout = CheckoutBuilder::new();
            // force the overwrite of all files
            checkout.force();
            repo.checkout_head(Some(&mut checkout))?;
            Ok(())
        } else {
            Err(Error::from_str(
                "Merge conflict or other reason, skipping pull",
            ))
        }
    }
}
