use serde::{Deserialize, Serialize, Serializer};


#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum BudgetRange {
  Under100k,
  From100kTo500k,
  From500kTo1m,
  From1mTo5m,
  Over5m
}

impl BudgetRange {
  pub fn from_str(role: &str) -> Result<BudgetRange, String> {
      match role {
          "Under $100k" => Ok(BudgetRange::Under100k),
          "$100k - $500k" => Ok(BudgetRange::From100kTo500k),
          "$500k - $1m" => Ok(BudgetRange::From100kTo500k),
          "$1m - $5m" => Ok(BudgetRange::From1mTo5m),
          "Over $5m" => Ok(BudgetRange::Over5m),
          _ => Err("Budget Range is not correct".to_string())
      }
  }

  pub fn as_str(&self) -> &'static str {
      match self {
        BudgetRange::Under100k =>  "Under $100k",
        BudgetRange::From100kTo500k =>  "$100k - $500k", 
        BudgetRange::From500kTo1m => "$500k - $1m",
        BudgetRange::From1mTo5m =>  "$1m - $5m",
        BudgetRange::Over5m =>  "Over $5m",
      }
  }
}


impl Serialize for BudgetRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}
