use chrono::prelude::*;
use dirs::{self};
use dotupdater::init;
use dotupdater::logger;
use git2::build::CheckoutBuilder;
use git2::{Error, FetchOptions, RemoteCallbacks, Repository};
use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use toml;

#[derive(Debug, serde::Deserialize)]
struct Config {
    repositories: Vec<RepositoryConfig>,
    config_base_path: String,
    log_path: String,
}

#[derive(Debug, serde::Deserialize)]
struct RepositoryConfig {
    path: String,
    branch: String,
}

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

fn fetch(repo_path: &str, branch: &str) -> Result<(), Error> {
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
fn pull(repo_path: &str, branch: &str) -> Result<(), Error> {
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

// check internet connection by querying Google DNS. Return a boolean if connected.
fn check_internet_connection() -> bool {
    TcpStream::connect("8.8.8.8:53").is_ok()
}

// function to implement an infinite cycle if the computer is connected. while the computer is not
// connected sleep for 3 second then re-check the connection.
fn wait_for_connection() {
    while !check_internet_connection() {
        sleep(Duration::from_secs(3));
    }
}

// create useful function for log. returns a string to be placed in log file.
fn get_task_datetime() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted_date
}

fn get_config_dir() -> Option<PathBuf> {
    let config_dir = dirs::config_dir();
    config_dir
}

fn get_user_home_dir() -> Option<PathBuf> {
    let homedir: Option<PathBuf> = dirs::home_dir();
    homedir
}

// I moved the pull function outside the main function to feel more confident and to make it easier
// to pass arguments and log the corresponding output.
fn main() {
    // retrieves config_dir
    let config_dir = get_config_dir().expect("Can't find config user dir.");
    // retrieves home_dir
    let homedir = get_user_home_dir().expect("Can't find home dir. Sure your OS is OK?");
    // read_to_string helps me to convert a stream from a file in a text to be read.
    let config_data = fs::read_to_string("config.toml")
        .expect("Unable to read TOML file. Is the correct path, isn't it?");
    // deserialize config_data starting from a simple string.
    let config: Config =
        toml::de::from_str(&config_data).expect("Unable to read single configurations.");
    // bind config.config.config_base_path to the variable config_base_path
    let config_base_path: String = config.config_base_path;
    // same story for logfiles_path
    let logfiles_path: String = format!("{}/{}", config.log_path, "logfile.log");
    // create log file if not exists, otherwise append logs to that file.
    let mut log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(logfiles_path)
        .expect("Cannot open log file.");
    // check_internet_connection
    wait_for_connection();
    // iterate over each repo in config files
    for repo_config in &config.repositories {
        let full_path: String = format!("{}{}", config_base_path, repo_config.path);
        let log_date: String = get_task_datetime();
        // write which repo is processing with date time information
        writeln!(
            log_file,
            "[I] {} - Processing repository: {}",
            log_date, full_path
        )
        .unwrap();
        // start to fetch at every repo
        match fetch(&full_path, &repo_config.branch) {
            Ok(_) => {
                let log_date: String = get_task_datetime();
                // if fetch result is: up-to-date skip and write process to logfile.
                writeln!(
                    log_file,
                    "[!] {} - No updates for branch {}",
                    log_date, &repo_config.branch
                );
            }
            Err(e) => {
                // fetch method retrieves some update, behave accordingly but, write on log.
                let log_date: String = get_task_datetime();
                writeln!(
                    log_file,
                    "[II] {} - Fetching updates for {}: {}",
                    log_date, &repo_config.branch, e
                );
                match pull(&full_path, &repo_config.branch) {
                    Ok(_) => {
                        let log_date: String = get_task_datetime();
                        writeln!(
                            log_file,
                            "[U] {} - Pulling updates for {}",
                            log_date, &repo_config.branch
                        );
                    }
                    Err(e) => {
                        let log_date: String = get_task_datetime();
                        writeln!(
                            log_file,
                            "[E] {} - Failed to pulling updates for {}: {}",
                            log_date, &repo_config.branch, e
                        );
                    }
                }
            }
        }
    }
}
