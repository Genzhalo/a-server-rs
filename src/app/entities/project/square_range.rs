use serde::{Deserialize, Serialize, Serializer};


#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum SquareRange {
  Under2th,
  From2thTo5th,
  From5thTo10th,
  From10thTo20th,
  From20thTo100th,
  Over100th
}

impl SquareRange {
  pub fn from_str(role: &str) -> Result<SquareRange, String> {
      match role {
          "Under 2,000 sqft" => Ok(SquareRange::Under2th),
          "2,000 - 5,000 sqft" => Ok(SquareRange::From2thTo5th),
          "5,000 - 10,000 sqft" => Ok(SquareRange::From5thTo10th),
          "10,000 - 20,000 sqft" => Ok(SquareRange::From10thTo20th),
          "20,000 - 100,000 sqft" => Ok(SquareRange::From20thTo100th),
          "Over 100,000 sqft" => Ok(SquareRange::Over100th),
          _ => Err("Square Range is not correct".to_string())
      }
  }

  pub fn as_str(&self) -> &'static str {
      match self {
        SquareRange::Under2th =>  "Under 2,000 sqft",
        SquareRange::From2thTo5th =>  "2,000 - 5,000 sqft", 
        SquareRange::From5thTo10th => "5,000 - 10,000 sqft",
        SquareRange::From10thTo20th =>  "10,000 - 20,000 sqft",
        SquareRange::From20thTo100th =>  "20,000 - 100,000 sqft",
        SquareRange::Over100th =>  "Over 100,000 sqft",
      }
  }
}


impl Serialize for SquareRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}
