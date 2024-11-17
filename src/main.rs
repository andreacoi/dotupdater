use git2::{Error, FetchOptions, RemoteCallbacks, Repository};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::net::TcpStream;
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
        Err(Error::from_str("Repository has updates available"))
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

// I moved the pull function inside the main function to feel more confident and to make it easier
// to pass arguments and log the corresponding output.
fn main() {
    // read_to_string helps me to convert a stream from a file in a text to be read.
    let config_data = fs::read_to_string("config.toml")
        .expect("Unable to read TOML file. Is the correct path, isn't it?");
    // deserialize config_data starting from a simple string.
    let config: Config =
        toml::de::from_str(&config_data).expect("Unable to read single configurations.");
    // bind config.config.config_base_path to the variable config_base_path
    let config_base_path: String = config.config_base_path;
    // same story for logfiles_path
    let logfiles_path: String = config.log_path;
    // create log file if not exists, otherwise append logs to that file.
    let mut log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(logfiles_path)
        .expect("Cannot open log file.");
    // iterate over each repo in config files
    for repo_config in &config.repositories {}
    // check_internet_connection
    wait_for_connection();
}
