// tip to use module init
//use dotupdater::init::initialize;
use dotupdater::{
    appvars::prepend_dir,
    ghutils::{self, get_config_list},
    init::initialize,
    logger::{logevent, EventType},
    utils::wait_for_connection,
};
use std::path::Path;
fn main() {
    initialize();
    let config = get_config_list();
    // cycle every config in the file config.toml
    for repo_config in &config.repositories {
        // cycle every repo config
        // get full path of the repo - repo usually located in $HOME/.config
        let full_path: String = prepend_dir(&repo_config.path);
        // log entering in a dir
        let _ = logevent(
            format!("Inspecting folder for a GitHub repo: {}", &full_path),
            EventType::I(String::from("[I]")),
        );
        // log if the folder specified in the config file does not exist.
        if !Path::new(&full_path).exists() {
            let _ = logevent(
                format!("Folder {} does not exist.\n", &full_path),
                EventType::E(String::from("[E]")),
            );
        } else {
            // log starting to fetch some available modifications
            let _ = logevent(
                format!("Fetching something new..."),
                EventType::I(String::from("[I]")),
            );
            // use ghutils.fetch to fetch some available modifications - manage that with OK and
            // Err
            // use function to check internet connection before any action on the repo
            wait_for_connection();
            let _ = match ghutils::fetch(&full_path, &repo_config.branch) {
                Ok(()) => logevent(
                    format!("No recent updates on this repo."),
                    EventType::W(String::from("[!!]")),
                ),
                Err(e) => {
                    let _ = logevent(format!("{e}"), EventType::I(String::from("[I]")));
                    // if the repo has updates pull them and log them in the log file
                    match ghutils::pull(&full_path, &repo_config.branch) {
                        Ok(()) => logevent(
                            format!(
                                "Pulling recent updates for repo: {} found in the branch: {}",
                                &full_path, &repo_config.branch
                            ),
                            EventType::I(String::from("[I]")),
                        ),
                        // if the repo can be pulled write that situation in the log file.
                        Err(e) => logevent(
                            format!("Pulling failed for some reason: {e} - Skipping updates."),
                            EventType::E(String::from("[E]")),
                        ),
                    }
                }
            };
        };
    }
}
