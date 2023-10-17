use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
