use chrono::{DateTime, Utc};
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::app::{
  entities::{ project::{ appropriate_status::AppropriateStatus, budget_range::BudgetRange, square_range::SquareRange, commercial_work::CommercialWork, architectural_service::ArchitecturalServices, Project, status::ProjectStatus}, user::user_type::UserType},
  errors::BaseError,
  traits::repositories::{notification::TNotificationRepositories, user::TUserRepositories, project::TProjectRepositories}, utils::validate_params::validate, email::project::ProjectEvents,
};

use super::user::UserService;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    #[validate(length(min = 2, message = "The name length should be min 2 symbols"))]
    pub name: String,
    #[validate(length(min = 2, message = "The city length should be min 2 symbols"))]
    pub city: String,
    pub street: Option<String>,
    pub zip_code: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub building_type: String,
    pub is_save_carbon: bool,
    #[validate(custom(function = "validate_appropriate_status", message = "Appropriate Status is invalid"))]
    pub appropriate_status: String,
    pub has_financing_secured: bool,
    #[validate(custom(function = "validate_budget_range", message = "Budget range is invalid"))]
    pub budget_range: String,
    #[validate(custom(function = "validate_square_range", message = "Square Range is invalid"))]
    pub square_range: String,
    #[validate(custom(function = "validate_commercial_work", message = "Commercial Work is invalid"))]
    pub commercial_work: String,
    #[validate(custom(function = "validate_architectural_services", message = "Architectural Services is invalid"))]
    pub architectural_services: Vec<String>,
    pub completion_date: DateTime<Utc>,
}


fn validate_appropriate_status(value: &str) -> Result<(), ValidationError> {
  match AppropriateStatus::from_str(value)  {
      Ok(_) => Ok(()),
      Err(_) =>  Err(ValidationError::new(""))
  }
}

fn validate_budget_range(value: &str) -> Result<(), ValidationError> {
  match BudgetRange::from_str(value)  {
      Ok(_) => Ok(()),
      Err(_) =>  Err(ValidationError::new(""))
  }
}

fn validate_square_range(value: &str) -> Result<(), ValidationError> {
  match SquareRange::from_str(value)  {
      Ok(_) => Ok(()),
      Err(_) =>  Err(ValidationError::new(""))
  }
}

fn validate_commercial_work(value: &str) -> Result<(), ValidationError> {
  match CommercialWork::from_str(value)  {
      Ok(_) => Ok(()),
      Err(_) =>  Err(ValidationError::new(""))
  }
}

fn validate_architectural_services(value: &Vec<String>) -> Result<(), ValidationError> {
  for  v in value {
    match ArchitecturalServices::from_str(&v)  {
      Ok(_) => {}
      Err(_) =>  return Err(ValidationError::new(""))
    }
  }
  Ok(())
}


#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectByQueryParams {
  limit: Option<i16>,
  skip: Option<i16>,
  #[validate(custom(function = "validate_status", message = "Project Status is invalid"))]
  status: Option<String>,
  name: Option<String>
}

fn validate_status(value: &str) -> Result<(), ValidationError> {
  match ProjectStatus::from_str(value)  {
      Ok(_) => Ok(()),
      Err(_) =>  Err(ValidationError::new(""))
  }
}


pub struct ProjectService<'a> {
  user_rep: &'a (dyn TUserRepositories + Send + Sync),
  user_service: UserService<'a>,
  notification_rep: &'a (dyn TNotificationRepositories + Send + Sync),
  project_rep:  &'a (dyn TProjectRepositories + Send + Sync),
}

impl<'a> ProjectService<'a> {
  pub fn new(
      user_rep: &'a (dyn TUserRepositories + Send + Sync),
      notification_rep: &'a (dyn TNotificationRepositories + Send + Sync),
      project_rep:  &'a (dyn TProjectRepositories + Send + Sync),
      token: &'a str
  ) -> Self {
      Self {
          user_rep,
          project_rep,
          notification_rep,
          user_service: UserService::new(user_rep, token),
      }
  }

  pub async fn create(&self, params: CreateParams) -> Result<String, BaseError> {
      match validate(&params) {
        Ok(_) => (),
        Err(e) => return Err(e),
      };

      let user = match self.user_service.get_current_user().await {
          Ok(user) => user,
          Err(err) => return Err(err),
      };

      if user.u_type != UserType::Client {
        return  Err(BaseError::new("Forbidden".to_owned()))
      }

      let project_id = match self.project_rep.insert(&user.id, ProjectStatus::Active, &params).await {
          Ok(id) => id,
          Err(e) => return Err(BaseError::new(e.to_string())),
      };

      let admins = self.user_rep.find(vec![UserType::Admin], None, None, None).await; 

      println!(">>>>>>>>>{:?}", admins);
      if admins.len() > 0 {
        let _ = self.notification_rep.insert(
          format!("create a project {}", params.name).as_str(), 
          format!("/projects/{project_id}").as_str(), 
          false, 
          false, 
          &user.id, 
          admins.iter().map( |a| a.id.as_str()).collect()
        ).await;

        let _ = ProjectEvents::default().on_create_project(
          user, 
          &params.name, 
          &project_id,
          admins.iter().map( |u| u.email.as_str()).collect()).await; 
      }

      Ok(project_id)
  }


  pub async fn get_by_id(&self, id: &str) -> Result<Project, BaseError> {
    match self.user_service.get_current_user().await {
        Ok(user) => user,
        Err(err) => return Err(err),
    };
    
    match self.project_rep.find_by_id(id).await {
        Some(project) => Ok(project),
        None => Err(BaseError::new("Not Found".to_string())),
    }
  }

  pub async fn get(&self, params: &GetProjectByQueryParams) -> Result<Vec<Project>, BaseError> {
    match validate(&params) {
      Ok(_) => (),
      Err(e) => return Err(e),
    };

    match self.user_service.get_current_user().await {
        Ok(user) => user,
        Err(err) => return Err(err),
    };

    Ok(self.project_rep.find_by_query(&params).await)
  }
}
