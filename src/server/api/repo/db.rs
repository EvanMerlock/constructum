use sqlx::PgPool;
use uuid::Uuid;

use super::RepoInfo;

pub async fn list_repos(
    pool: PgPool
) -> Result<Vec<RepoInfo>, sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query_as("SELECT * FROM constructum.repositories").fetch_all(&mut sql_connection).await
}

#[tracing::instrument]
pub async fn get_repo(
    repo_id: Uuid,
    pool: PgPool
) -> Result<RepoInfo, sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query_as("SELECT * FROM constructum.repositories WHERE id = $1")
        .bind(repo_id)
        .fetch_one(&mut sql_connection)
        .await
}

#[tracing::instrument]
pub async fn get_repo_optional(
    repo_id: Uuid,
    pool: PgPool
) -> Result<Option<RepoInfo>, sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query_as("SELECT * FROM constructum.repositories WHERE id = $1")
        .bind(repo_id)
        .fetch_optional(&mut sql_connection)
        .await
}


pub async fn get_repo_by_git_id(
    git_repo_id: i32,
    pool: PgPool
) -> Result<Option<RepoInfo>, sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query_as("SELECT * FROM constructum.repositories WHERE git_id = $1")
        .bind(git_repo_id)
        .fetch_optional(&mut sql_connection)
        .await
}

#[tracing::instrument]
pub async fn register_repo(
    pool: PgPool,
    payload: RepoInfo
) -> Result<(), sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query("INSERT INTO constructum.repositories (id, git_id, repo_url, repo_owner, repo_name, webhook_id, enabled, builds_executed) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(payload.repo_uuid)
        .bind(payload.git_id)
        .bind(payload.repo_url)
        .bind(payload.repo_owner)
        .bind(payload.repo_name)
        .bind(payload.webhook_id)
        .bind(payload.enabled)
        .bind(payload.builds_executed)
        .execute(&mut sql_connection).await?;
    Ok(())
}

#[tracing::instrument]
pub async fn delete_repo(
    repo_id: Uuid,
    pool: PgPool
) -> Result<(), sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    // TODO: soft delete instead of hard delete. will need to check for that on re-enable.
    sqlx::query("UPDATE constructum.repositories SET enabled = false, webhook_id = NULL WHERE id = $1")
        .bind(repo_id)
        .execute(&mut sql_connection)
        .await?;
    Ok(())
}

#[tracing::instrument]
pub async fn enable_repo(
    repo_id: Uuid,
    webhook_id: i32,
    pool: PgPool
) -> Result<(), sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    // TODO: soft delete instead of hard delete. will need to check for that on re-enable.
    sqlx::query("UPDATE constructum.repositories SET enabled = true, webhook_id = $2 WHERE id = $1")
        .bind(repo_id)
        .bind(webhook_id)
        .execute(&mut sql_connection)
        .await?;
    Ok(())
}

#[tracing::instrument]
pub async fn update_repo_seq(
    pool: PgPool,
    repo_id: Uuid,
    last_seq: i32,
) -> Result<(), sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    // TODO: soft delete instead of hard delete. will need to check for that on re-enable.
    sqlx::query("UPDATE constructum.repositories SET builds_executed = $2 WHERE id = $1")
        .bind(repo_id)
        .bind(last_seq)
        .execute(&mut sql_connection)
        .await?;
    Ok(())
}