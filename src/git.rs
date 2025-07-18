use crate::types::GitProfile;
use std::process::Command;

pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn get_current_git_user() -> GitProfile {
    let name = run_git_config("user.name");
    let email = run_git_config("user.email");
    GitProfile { name, email }
}

fn run_git_config(key: &str) -> String {
    String::from_utf8(
        Command::new("git")
            .args(["config", "--get", key])
            .output()
            .expect("Git not installed")
            .stdout,
    )
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn set_git_user(profile: &GitProfile) {
    println!(
        "Setting {} as user.name\nSetting {} as user.email",
        profile.name, profile.email
    );

    Command::new("git")
        .args(["config", "user.name", &profile.name])
        .status()
        .unwrap();

    Command::new("git")
        .args(["config", "user.email", &profile.email])
        .status()
        .unwrap();

    Command::new("git")
        .args(["config", "--unset", "user.signingKey"])
        .status()
        .ok(); // In case it's not set
}
