use serde::{Deserialize, Serialize, Serializer};


#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ProjectStatus {
  Draft,
  Active,
  Ongoing,
  Delivered,
  Overdue,
  Complete,
  Withdrawn,
  OnHold
}

impl ProjectStatus {
  pub fn from_str(role: &str) -> Result<ProjectStatus, String> {
      match role {
          "Draft" => Ok(ProjectStatus::Draft),
          "Active" => Ok(ProjectStatus::Active),
          "Ongoing" => Ok(ProjectStatus::Ongoing),
          "Delivered" => Ok(ProjectStatus::Delivered),
          "Overdue" => Ok(ProjectStatus::Overdue),
          "Complete" => Ok(ProjectStatus::Complete),
          "Withdrawn" => Ok(ProjectStatus::Withdrawn),
          "OnHold" => Ok(ProjectStatus::OnHold),
          _ => Err("Project Status is not correct".to_string())
      }
  }

  pub fn as_str(&self) -> &'static str {
      match self {
        ProjectStatus::Draft => "Draft",
        ProjectStatus::Active => "Active", 
        ProjectStatus::Ongoing => "Ongoing",
        ProjectStatus::Delivered => "Delivered",
        ProjectStatus::Overdue => "Overdue",
        ProjectStatus::Complete => "Complete",
        ProjectStatus::Withdrawn => "Withdrawn",
        ProjectStatus::OnHold => "OnHold",
      }
  }
}


impl Serialize for ProjectStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}
