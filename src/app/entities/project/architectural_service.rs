use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ArchitecturalServices {
    Space,
    ArchitectOrdDesigner,
    Drawings ,
    SubmittedForAPermit ,
    ApprovedForAPermit,
    Contractor ,
    ReceivedPricingFromAContractor ,
    NoTeamOrDesign  
}

impl ArchitecturalServices {
    pub fn from_str(role: &str) -> Result<ArchitecturalServices, String> {
        match role {
            "I have the space (rent or own)" => Ok(ArchitecturalServices::Space),
            "I have an architect or designer" => Ok(ArchitecturalServices::ArchitectOrdDesigner),
            "I have drawings of my design" => Ok(ArchitecturalServices::Drawings),
            "The project has been submitted for a permit" => Ok(ArchitecturalServices::SubmittedForAPermit),
            "I have a contractor" => Ok(ArchitecturalServices::Contractor),
            "I have received pricing from a contractor" => Ok(ArchitecturalServices::ReceivedPricingFromAContractor),
            "I know what I want but do not have the team or design yet" => Ok(ArchitecturalServices::NoTeamOrDesign),
            "The project has been approved for a permit" =>    Ok(ArchitecturalServices::ApprovedForAPermit),
            _ => Err("Architectural Service is not correct".to_string())
        }
    }

    pub fn as_str(&self) -> &'static str {
      match self {
        ArchitecturalServices::Space => "I have the space (rent or own)",
        ArchitecturalServices::ArchitectOrdDesigner => "I have an architect or designer",
        ArchitecturalServices::Drawings => "I have drawings of my design",
        ArchitecturalServices::SubmittedForAPermit => "The project has been submitted for a permit",
        ArchitecturalServices::Contractor => "I have a contractor",
        ArchitecturalServices::ReceivedPricingFromAContractor => "I have received pricing from a contractor",
        ArchitecturalServices::NoTeamOrDesign => "I know what I want but do not have the team or design yet)",
        ArchitecturalServices::ApprovedForAPermit => "The project has been approved for a permit",
      }
  }
}


impl Serialize for ArchitecturalServices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}
