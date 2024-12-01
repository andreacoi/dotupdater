use crate::appvars::{get_complete_config_file_path, CONFIG_FILE};
use crate::config::{Config, RepositoryConfig};
use git2::build::CheckoutBuilder;
use git2::{Error, FetchOptions, RemoteCallbacks, Repository};
use std::fs;

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

pub fn get_config_list() -> Config {
    let config_file_path = get_complete_config_file_path();
    // get complete config file path
    let config_file: String = format!("{}/{}", config_file_path, CONFIG_FILE);
    // read_to_string helps me to convert a stream from a file in a text to be read.
    let config_data = match fs::read_to_string(config_file) {
        Ok(config_data) => config_data,
        Err(_) => String::from("Error"),
    };
    // deserialize config_data starting from a simple string.
    let config: Config =
        toml::de::from_str(&config_data).expect("Unable to read single configurations.");
    config
}
