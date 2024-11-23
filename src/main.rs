use dirs;
use dotupdater::github_utils::{fetch, pull};
use dotupdater::init;
use dotupdater::logger;
use dotupdater::utils::wait_for_connection;
use dotupdater::Config;
use dotupdater::RepositoryConfig;

// I moved the pull function outside the main function to feel more confident and to make it easier
// to pass arguments and log the corresponding output.
fn main() {}
