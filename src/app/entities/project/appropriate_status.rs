use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum AppropriateStatus {
  ReadyToHire,
  Researching,
}


impl AppropriateStatus {
  pub fn from_str(value: &str) -> Result<AppropriateStatus, String> {
    match value {
        "Ready to Hire" => Ok(AppropriateStatus::ReadyToHire),
        "Researching" => Ok(AppropriateStatus::Researching),
        _ => Err("Appropriate Status is not correct".to_string())
    }
}

  pub fn as_str(&self) -> &'static str {
      match self {
        AppropriateStatus::ReadyToHire => "Ready to Hire",
        AppropriateStatus::Researching => "Researching",
      }
  }

}


impl Serialize for AppropriateStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}
