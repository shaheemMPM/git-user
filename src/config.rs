use crate::types::{GitProfile, GitConfig};
use dirs::config_dir;
use std::{path::PathBuf};
use std::fs::{create_dir_all, read_to_string, write};
use toml::{from_str, to_string};

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().unwrap();
    path.push("git-user");
    create_dir_all(&path).unwrap();
    path.push("config.toml");
    path
}

pub fn load_profiles(path: &PathBuf) -> Vec<GitProfile> {
    match read_to_string(path) {
        Ok(content) => {
            match from_str::<GitConfig>(&content) {
                Ok(config) => config.profiles,
                Err(_) => Vec::new(),
            }
        }
        Err(_) => Vec::new(),
    }
}

pub fn save_profiles(path: &PathBuf, profiles: &Vec<GitProfile>) {
    let config = GitConfig {
        profiles: profiles.clone(),
    };
    let toml_data = to_string(&config).unwrap();
    write(path, toml_data).unwrap();
}
