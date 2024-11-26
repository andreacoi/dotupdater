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

    pub fn get_complete_log_file_path() -> String {
        let complete_log_file_path: String = format!("{}{}", LOGDIR, LOGFILE);
        complete_log_file_path
    }

    pub fn get_complete_config_file_path() -> String {
        let complete_config_file_path: String = format!(
            "{}/{}",
            get_config_dir().unwrap().to_str().unwrap().to_owned(),
            CONFIG_FILE
        );
        complete_config_file_path
    }
}

pub mod init {
    use std::fs::{self, OpenOptions};
    use std::io::{self, Write};

    use chrono::format;
    use toml::ser::Error;

    use crate::logger::logevent;

    fn create_base_files_with_content(
        path: String,
        filename: String,
        content: String,
    ) -> Result<(), String> {
        // create complete path concatenating String path and String filename
        let complete_file_path = format!("{}/{}", &path, &filename);
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&complete_file_path);

        if Path::new(&complete_file_path).exists() {
            return Err(format!("File {} already exists.", &complete_file_path));
        } else {
            match file {
                Ok(mut file) => {
                    write!(file, "{}", &content);
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                    logevent(
                        String::from("Notice: File already exists, you can use app modifying files in the proper directory."),
                        crate::logger::EventType::N(String::from("Notice")),
                    );
                }
                Err(e) => {
                    logevent(
                        String::from("Error: File cannot be created in the specified directory."),
                        crate::logger::EventType::E(String::from("Error")),
                    );
                }
            }
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
        // get config_dir by enviroment
        // !!!! todo: manage existence of config folder and config file
        // tip: if the folder does not exist then even the file can't exist (logically)
        // fn create_config_folder -> Ok(folder_name)
        // fn create_config_file -> Ok(())
        // get program folder - dotupdater in this case - e.g. /home/johndoe/.config/dotupdater
        let dufolder: &str = APP_NAME;
        // build complete path to be passed to create folder function
        let complete_app_path: String = format!("{}/{}", &opt_path, &dufolder);
        // get logpath, set statically and of imperium to /var/tmp/
        let logpath: String = String::from("/var/tmp/");
        // build complete log path to be passed to create folder function
        let complete_log_path: String = format!("{}/{}_logs", &logpath, &dufolder);
        // function to create log folder and log file
        match fs::create_dir_all(&complete_log_path) {
            Ok(()) => {
                create_base_files_with_content(
                    complete_log_path,
                    String::from("dotupdater.log"),
                    String::from("Created log file for the first time."),
                );
            }
            Err(e) => println!("Unable to create log folder for some reason: {e}"),
        }
        // function to create program config folder
        match fs::create_dir_all(&complete_app_path) {
            Ok(_) => {
                match create_base_files_with_content(
                    complete_app_path,
                    String::from(CONFIG_FILE),
                    String::from(BLUEPRINT_FILE),
                ) {
                    Ok(_) => {
                        logevent(
                            String::from(
                                "Created blueprint file, modify it in order to use correctly",
                            ),
                            crate::logger::EventType::N(String::from("Notice")),
                        );
                    }
                    Err(_) => println!("test"),
                };
            }

            // n config_folder... BLABLABLA
            Err(e) => println!("{:?}", &e), //call logger - some like... unable to create
                                            // config folder because of e.
        }

        // create some config file in order to suppress errors
        // read config file
        // get home_dir
        // get config_dir
        // get log_dir
    }
}

pub mod logger {
    use chrono::prelude::*;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    // Returns a string to be placed in log file.
    fn get_task_datetime() -> String {
        let local: DateTime<Local> = Local::now();
        let formatted_date = local.format("%Y-%m-%d %H:%M:%S").to_string();
        formatted_date
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

    pub fn initialize_log_file() -> bool {
        let logfile = crate::appvars::get_complete_log_file_path();
        if Path::new(&logfile).exists() == true {
            true
        }
        false
    }

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

        let mut file = OpenOptions::new().append(true).create(true).open(LOGFILE)?;
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
