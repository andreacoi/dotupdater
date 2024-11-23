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

    // create some config file in order to suppress errors
    // read config file
    // get home_dir
    // get config_dir
    // get log_dir
}

pub mod logger {
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
}
