use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserType {
    Client,
    Vendor,
    Admin,
}

impl UserType {
    pub fn from_str(role: &str) -> UserType {
        match role {
            "Admin" => UserType::Admin,
            "Client" => UserType::Client,
            "Vendor" => UserType::Vendor,
            _ => panic!(),
        }
    }
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserType::Client => write!(f, "Client"),
            UserType::Vendor => write!(f, "Vendor"),
            UserType::Admin => write!(f, "Admin"),
        }
    }
}
