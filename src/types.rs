use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GitProfile {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitConfig {
    pub profiles: Vec<GitProfile>,
}

pub enum ProfileSelection {
    Selected(GitProfile),
    AddNew,
    Cancelled,
}