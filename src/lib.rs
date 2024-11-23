#[derive(Debug, serde::Deserialize)]
pub struct Config {
    repositories: Vec<RepositoryConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RepositoryConfig {
    path: String,
    branch: String,
}

pub mod init {
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use toml;

    const APP_NAME: &str = "dotupdater";

    fn get_config_dir() -> Option<PathBuf> {
        let config_dir = dirs::config_dir();
        config_dir
    }

    fn get_user_home_dir() -> Option<PathBuf> {
        let homedir: Option<PathBuf> = dirs::home_dir();
        homedir
    }

    // this function create all the enviroment.
    pub fn initialize() {
        // read app config file - located in user dir/.config/dotupdater
    }

    // create some config file in order to suppress errors
    // read config file
    // get home_dir
    // get config_dir
    // get log_dir
}

pub mod logger {
    use chrono::prelude::*;
    // create useful function for log. returns a string to be placed in log file.

    fn get_task_datetime() -> String {
        let local: DateTime<Local> = Local::now();
        let formatted_date = local.format("%Y-%m-%d %H:%M:%S").to_string();
        formatted_date
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
