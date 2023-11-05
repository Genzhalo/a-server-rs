use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::{appropriate_status::AppropriateStatus, architectural_service::ArchitecturalServices, budget_range::BudgetRange, square_range::SquareRange, commercial_work::CommercialWork, status::ProjectStatus};

use super::user::User;

pub mod architectural_service;
pub mod appropriate_status;
pub mod budget_range;
pub mod commercial_work;
pub mod square_range;
pub mod status;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub city: String,
    pub status: ProjectStatus,
    pub street: Option<String>,
    pub zip_code: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub building_type: String,
    pub is_save_carbon: bool,
    pub appropriate_status: AppropriateStatus,
    pub has_financing_secured: bool,
    pub budget_range: BudgetRange,
    pub square_range: SquareRange,
    pub commercial_work: CommercialWork,
    pub architectural_services: Vec<ArchitecturalServices>,
    pub completion_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub creator: User
}
