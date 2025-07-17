use crate::types::GitProfile;
use inquire::{Select, Text};

pub fn select_profile(profiles: &Vec<GitProfile>) -> Option<GitProfile> {
    let mut options: Vec<String> = profiles
        .iter()
        .map(|p| format!("{} : {}", p.name, p.email))
        .collect();

    options.push("Add new user".to_string());

    let selection = Select::new("Select a git user", options).prompt();

    match selection {
        Ok(choice) if choice == "Add new user" => None,
        Ok(choice) => {
            let index = profiles
                .iter()
                .position(|p| format!("{} : {}", p.name, p.email) == choice)
                .unwrap();
            Some(profiles[index].clone())
        }
        Err(_) => None,
    }
}

pub fn prompt_new_profile() -> GitProfile {
    let name = Text::new("Enter your git user name").prompt().unwrap();
    let email = Text::new("Enter your git user email").prompt().unwrap();
    GitProfile { name, email }
}
