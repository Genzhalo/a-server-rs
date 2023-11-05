use async_trait::async_trait;

use crate::app::{services::project::{CreateParams, GetProjectByQueryParams}, entities::project::{Project, status::ProjectStatus}};

#[async_trait]
pub trait TProjectRepositories {
    async fn insert(&self, user_id: &str, status: ProjectStatus, data: &CreateParams) -> Result<String, String>;
    async fn find_by_id(&self, id: &str) -> Option<Project>;
    async fn find_by_query(&self, query: &GetProjectByQueryParams) -> Vec<Project>;
}
