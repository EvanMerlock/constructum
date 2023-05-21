use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;



#[derive(Debug, Serialize)]
pub struct RepoInfo {
    pub repo_uuid: Uuid,
    pub git_id: i32,
    pub repo_url: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub webhook_id: i32,
}

impl<'r> sqlx::FromRow<'r, PgRow> for RepoInfo {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let uuid: Uuid = row.try_get("id")?;
        let git_id: i32 = row.try_get("git_id")?;
        let repo_url: String = row.try_get("repo_url")?;
        let repo_owner: String = row.try_get("repo_owner")?;
        let repo_name: String = row.try_get("repo_name")?;
        let webhook_id: i32 = row.try_get("webhook_id")?;

        Ok(
            RepoInfo { repo_uuid: uuid, git_id, repo_url, repo_owner, repo_name, webhook_id }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GiteaRepository {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub html_url: String,
    pub ssh_url: String,
    pub owner: GiteaUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GiteaUser {
    pub id: i32,
    pub login: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRepositoryPayload {
    pub owner: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitRepoResponse {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub html_url: String,
    pub ssh_url: String,
    pub owner: GiteaUser,
    pub is_registed: bool,
}