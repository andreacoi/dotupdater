// tip to use module init
//use dotupdater::init::initialize;
use dotupdater::{
    appvars::prepend_dir,
    ghutils::{self, get_config_list},
    init::initialize,
    logger::{logevent, EventType},
};
fn main() {
    initialize();
    let config = get_config_list();
    // cycle every config in the file config.toml
    for repo_config in &config.repositories {
        let full_path: String = prepend_dir(&repo_config.path);
        logevent(
            format!("Inspecting folder for a GitHub repo: {}", &full_path),
            EventType::I(String::from("[I]")),
        );
    }
}
