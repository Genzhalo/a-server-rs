use serde::{Deserialize, Serialize, Serializer};


#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum CommercialWork {
  Renovation,
  SelectingFurnitureAndEquipment,
  EvaluatingForSustainability,
  AllOfTheAbove,
}

impl CommercialWork {
  pub fn from_str(role: &str) -> Result<CommercialWork, String>{
      match role {
          "Renovation" => Ok(CommercialWork::Renovation),
          "Selecting furniture and equipment" => Ok(CommercialWork::SelectingFurnitureAndEquipment),
          "Evaluating for sustainability" => Ok(CommercialWork::EvaluatingForSustainability),
          "All of the above" => Ok(CommercialWork::AllOfTheAbove),
          _ => Err("Commercial Work is not correct".to_string())
      }
  }

  pub fn as_str(&self) -> &'static str {
      match self {
        CommercialWork::Renovation =>  "Renovation",
        CommercialWork::SelectingFurnitureAndEquipment =>  "Selecting furniture and equipment", 
        CommercialWork::EvaluatingForSustainability => "Evaluating for sustainability",
        CommercialWork::AllOfTheAbove =>  "All of the above",
      }
  }
}


impl Serialize for CommercialWork {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}