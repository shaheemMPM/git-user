use crate::types::{GitProfile, ProfileSelection};
use inquire::{Select, Text};

pub fn select_profile(profiles: &Vec<GitProfile>) -> ProfileSelection {
    let mut options: Vec<String> = profiles
        .iter()
        .map(|p| format!("{} : {}", p.name, p.email))
        .collect();

    options.push(" ➕ Add new user".to_string());
    options.push("👋  Quit".to_string());

    let selection = Select::new("Select a git user", options).prompt();

    match selection {
        Ok(choice) if choice == " ➕ Add new user" => ProfileSelection::AddNew,
        Ok(choice) if choice == "👋  Quit" => ProfileSelection::Cancelled,
        Ok(choice) => {
            let profile = profiles
                .iter()
                .find(|p| format!("{} : {}", p.name, p.email) == choice)
                .cloned();
            match profile {
                Some(p) => ProfileSelection::Selected(p),
                None => ProfileSelection::Cancelled,
            }
        }
        Err(_) => ProfileSelection::Cancelled,
    }
}

pub fn prompt_new_profile() -> Option<GitProfile> {
    let name = Text::new("Enter your git user name").prompt().ok()?;
    let email = Text::new("Enter your git user email").prompt().ok()?;
    Some(GitProfile { name, email })
}
