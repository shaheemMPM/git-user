mod config;
mod git;
mod ui;
mod types;

use config::{get_config_path, load_profiles, save_profiles};
use git::{is_git_repo, get_current_git_user, set_git_user};
use ui::{select_profile, prompt_new_profile};

fn main() {
    if !is_git_repo() {
        eprintln!(" ❌ Not a Git repository. Please run this command from the root of a Git project.");
        std::process::exit(1);
    }

    let config_path = get_config_path();
    let mut profiles = load_profiles(&config_path);

    let current = get_current_git_user();
    println!("Current git user (local) is {}:{}", current.name, current.email);

    match select_profile(&profiles) {
        Some(profile) => {
            set_git_user(&profile);
        }
        None => {
            let new_profile = prompt_new_profile();
            set_git_user(&new_profile);
            profiles.push(new_profile);
            save_profiles(&config_path, &profiles);
        }
    }
}
