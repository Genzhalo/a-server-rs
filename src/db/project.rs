use crate::app::{
  entities::{project::{Project, appropriate_status::AppropriateStatus, budget_range::BudgetRange, square_range::SquareRange, commercial_work::CommercialWork, architectural_service::ArchitecturalServices, status::ProjectStatus}, user::{user_type::UserType, User}},
 services::project::{CreateParams, GetProjectByQueryParams}, traits::repositories::project::TProjectRepositories,
};
use async_trait::async_trait;
use std::{sync::Arc, time::SystemTime, ops::Deref};
use tokio_postgres::{Client, Row};

use super::from_row::base_user_from_row;

const USER_FIELDS: &str = "e.email AS user_email, 
    u.id AS user_user_id, 
    u.first_name AS user_first_name, 
    u.last_name AS user_last_name, 
    u.created_at AS user_created_at,
    u.type AS user_type";

impl Project {
  fn from_row(row: &Row) -> Self {
      Project { 
        id: row.get::<&str, String>("id"), 
        name: row.get::<&str, String>("name"), 
        city: row.get::<&str, String>("city"), 
        street: row.get::<&str, Option<String>>("street"), 
        zip_code: row.get::<&str, Option<String>>("zip_code"), 
        floor: row.get::<&str, Option<String>>("floor"),  
        description: row.get::<&str, Option<String>>("description"), 
        building_type: row.get::<&str, String>("building_type"), 
        is_save_carbon:  row.get::<&str, bool>("is_save_carbon"), 
        appropriate_status: AppropriateStatus::from_str( row.get::<&str, &str>("appropriate_status")).unwrap(), 
        has_financing_secured:row.get::<&str, bool>("has_financing_secured"),  
        budget_range: BudgetRange::from_str(row.get::<&str, &str>("budget_range")).unwrap(), 
        square_range: SquareRange::from_str(row.get::<&str, &str>("square_range")).unwrap(), 
        commercial_work: CommercialWork::from_str(row.get::<&str, &str>("commercial_work")).unwrap(), 
        architectural_services: row.get::<&str, Vec<&str>>("architectural_services").into_iter().map( |s|  ArchitecturalServices::from_str(s).unwrap()).collect::<Vec<ArchitecturalServices>>(), 
        completion_date: row.get::<&str, SystemTime>("completion_date").into(), 
        created_at: row.get::<&str, SystemTime>("created_at").into(),
        creator: base_user_from_row(row, "user"),
        status: ProjectStatus::from_str(row.get::<&str, &str>("status")).unwrap(),
    } 
       
  }
}

pub struct ProjectRepository {
  client: Arc<Client>,
}

impl ProjectRepository {
  pub fn new(client: Arc<Client>) -> Self {
      Self { client }
  }
}

#[async_trait]
impl TProjectRepositories for ProjectRepository {
  async fn insert( &self, user_id: &str, status: ProjectStatus, param: &CreateParams) -> Result<String, String> {
      let statement =
          "INSERT INTO project (
            name, 
            city, 
            street, 
            zip_code, 
            floor, 
            description, 
            building_type, 
            is_save_carbon, 
            appropriate_status, 
            has_financing_secured, 
            budget_range, 
            commercial_work, 
            architectural_services, 
            completion_date, 
            square_range,
            user_id,
            status
          ) 
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) 
            RETURNING id";

      let res = self
          .client
          .query_one(
            statement,
              &[
                  &param.name,
                  &param.city,
                  &param.street,
                  &param.zip_code,
                  &param.floor,
                  &param.description,
                  &param.building_type,
                  &param.is_save_carbon,
                  &param.appropriate_status.deref(),
                  &param.has_financing_secured,
                  &param.budget_range.deref(),
                  &param.commercial_work,
                  &param.architectural_services,
                  &param.completion_date.naive_utc(),
                  &param.square_range.deref(),
                  &user_id,
                  &status.as_str()
              ],
          )
          .await;


      match res {
          Ok(row) => Ok(row.get::<&str, String>("id")),
          Err(err) => match err.as_db_error() {
              Some(err) => Err(err.message().to_string()),
              None => Err(err.to_string()),
          },
      }
  }

  async fn find_by_id(&self, id: &str) -> Option<Project> {
    let statement = format!("
      SELECT p.*, {USER_FIELDS} 
        FROM project AS p
          JOIN users AS u ON p.user_id = u.id AND p.id = $1
          JOIN user_emails AS e ON e.user_id = p.user_id AND e.is_primary = true;");

    let res = self
        .client
        .query_one(&statement, &[&id])
        .await;
    
    match res {
        Ok(row) => Some(Project::from_row(&row)),
        Err(_) => None
    }
    
  }

  async fn find_by_query(&self, params: &GetProjectByQueryParams) -> Vec<Project> {
    return vec![];
    // let user_types: Vec<String> = types.iter().map( | t | t.to_string()).collect();
    // let value = format!("%{}%", value.unwrap_or("").split_whitespace().collect::<String>()).to_lowercase();
    // let statement =
    //     format!("SELECT u.*, e.email as email FROM users AS u 
    //         JOIN user_emails AS e ON u.id = e.user_id AND u.type = ANY($1) AND lower(CONCAT(u.first_name, u.last_name)) LIKE ($2)
    //         LIMIT $3 OFFSET $4;");

    // let res = self.client.query(&statement, &[&user_types, &value, &limit, &skip]).await;
    // match res {
    //     Ok(rows) => rows
    //         .into_iter()
    //         .map(|row| User::from_rows(&vec![row]))
    //         .collect(),
    //     Err(_) =>  vec![]
    // }
  }
  
}
