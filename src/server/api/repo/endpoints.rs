use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::{
    server::{
        api::{repo::{db::list_repos, GitRepoResponse}, job::JobInfo},
        error::ConstructumServerError,
    },
    utils, ConstructumServerState,
};

use super::{GiteaRepository, RegisterRepositoryPayload, RepoInfo};

pub async fn list_known_repos(
    State(state): State<ConstructumServerState>,
) -> Result<Json<Vec<RepoInfo>>, ConstructumServerError> {
    let repo_info = super::db::list_repos(state.postgres()).await?;

    Ok(Json(repo_info))
}

pub async fn list_all_repos(
    headers: HeaderMap,
    State(state): State<ConstructumServerState>,
) -> Result<Json<Vec<GitRepoResponse>>, ConstructumServerError> {
    let auth_tok = headers
        .get("Authorization")
        .ok_or(ConstructumServerError::BadAuthorization)?;
    let resp = utils::get_with_auth(
        format!("{}/api/v1/repos/search", state.git_server_url()),
        "Authorization",
        auth_tok.to_str()?.to_owned(),
    )
    .await?;

    #[derive(Debug, Deserialize)]
    struct GiteaRepositoryResponse {
        data: Vec<GiteaRepository>,
        ok: bool,
    }

    let git_repos_resp: GiteaRepositoryResponse = resp.json().await?;

    if !git_repos_resp.ok {
        // TODO: forgot to do this
        todo!()
    }

    let git_repos = git_repos_resp.data;
    let known_repos = list_repos(state.postgres()).await?;

    // TODO: this is O(n^2)
    let git_repos = git_repos
        .into_iter()
        .map(|x| {
            let known_repo: Option<&RepoInfo> = known_repos.iter().find(|y| y.git_id == x.id);

            let is_reg = match known_repo {
                Some(repo) => repo.enabled,
                None => false,
            };

            GitRepoResponse {
                id: known_repo.map(|x| x.repo_uuid),
                name: x.name,
                description: x.description,
                html_url: x.html_url,
                ssh_url: x.ssh_url,
                owner: x.owner,
                is_registered: is_reg,
            }
        })
        .collect();

    Ok(Json(git_repos))
}

pub async fn get_repo(
    Path(repo_id): Path<Uuid>,
    State(state): State<ConstructumServerState>,
) -> Result<Json<RepoInfo>, ConstructumServerError> {
    let repo_info = super::db::get_repo(repo_id, state.postgres()).await?;

    Ok(Json(repo_info))
}

#[tracing::instrument(skip(state, headers))]
pub async fn register_repository(
    headers: HeaderMap,
    State(state): State<ConstructumServerState>,
    Json(payload): axum::extract::Json<RegisterRepositoryPayload>,
) -> Result<impl IntoResponse, ConstructumServerError> {
    #[derive(Serialize)]
    struct RegisterRepositoryResponse {
        uuid: Uuid,
    }

    let auth_tok = headers
        .get("Authorization")
        .ok_or(ConstructumServerError::BadAuthorization)?;
    let corr_auth_tok = auth_tok.to_str()?.to_owned();
    let resp = utils::get_with_auth(
        format!(
            "{}/api/v1/repos/{}/{}",
            state.git_server_url(), payload.owner, payload.name
        ),
        "Authorization",
        corr_auth_tok.clone(),
    )
    .await?;

    let git_repo: GiteaRepository = resp.json().await?;

    // checking for existence
    match super::db::get_repo_by_git_id(git_repo.id, state.postgres()).await? {
        Some(RepoInfo {
            repo_uuid: _,
            git_id: _,
            repo_url: _,
            repo_owner: _,
            repo_name: _,
            webhook_id: _,
            enabled,
            builds_executed: _,
        }) if enabled => Err(ConstructumServerError::RepoAlreadyRegistered),
        Some(RepoInfo {
            repo_uuid,
            git_id: _,
            repo_url: _,
            repo_owner: _,
            repo_name: _,
            webhook_id: _,
            enabled,
            builds_executed: _,
        }) if !enabled => {
            // just disabled
            // create wh and input
            let wh_id = super::system::add_constructum_webhook(
                state.git_server_url(),
                git_repo.clone(),
                corr_auth_tok.clone(),
            )
            .await?;

            super::db::enable_repo(repo_uuid, wh_id, state.postgres()).await?;

            Ok((
                StatusCode::OK,
                serde_json::to_string(&RegisterRepositoryResponse { uuid: repo_uuid })
                    .expect("failed to serialize response"),
            ))
        }
        Some(_) => {
            panic!("should not happen")
        }
        None => {
            // need to know wh_id before adding repo to DB
            // TODO: considering reordering
            let wh_id = super::system::add_constructum_webhook(
                state.git_server_url(),
                git_repo.clone(),
                corr_auth_tok.clone(),
            )
            .await?;

            let repo_uuid = Uuid::new_v4();

            let payload = super::RepoInfo {
                repo_uuid,
                git_id: git_repo.id,
                repo_url: git_repo.html_url.clone(),
                repo_owner: git_repo.owner.login.clone(),
                repo_name: git_repo.name.clone(),
                webhook_id: Some(wh_id),
                enabled: true,
                builds_executed: 0,
            };

            super::db::register_repo(state.postgres(), payload).await?;

            // TODO: mark this as json
            Ok((
                StatusCode::OK,
                serde_json::to_string(&RegisterRepositoryResponse { uuid: repo_uuid })
                    .expect("failed to serialize response"),
            ))
        }
    }
}

pub async fn remove_repository(
    headers: HeaderMap,
    Path(repo_id): Path<Uuid>,
    State(state): State<ConstructumServerState>,
) -> Result<impl IntoResponse, ConstructumServerError> {
    // checking for existence
    let _repo_ref = super::db::get_repo_optional(repo_id, state.postgres())
        .await?
        .ok_or(ConstructumServerError::NoRepoFound)?;

    let auth_tok = headers
        .get("Authorization")
        .ok_or(ConstructumServerError::BadAuthorization)?;
    let corr_auth_tok = auth_tok.to_str()?.to_owned();

    let repo_info = super::db::get_repo(repo_id, state.postgres()).await?;

    super::system::remove_constructum_webhook(
        state.git_server_url(),
        repo_info,
        corr_auth_tok.clone(),
    )
    .await?;
    match super::db::delete_repo(repo_id, state.postgres()).await {
        Ok(()) => {}
        Err(e) => tracing::error!("Error: {}", e),
    };

    Ok((StatusCode::NO_CONTENT, ""))
}

#[tracing::instrument(skip(state))]
pub async fn jobs_for_repository(
    Path(repo_id): Path<Uuid>,
    State(state): State<ConstructumServerState>,
) -> Result<Json<Vec<JobInfo>>, ConstructumServerError> {
        // checking for existence
        let _repo_ref = super::db::get_repo_optional(repo_id, state.postgres())
        .await?
        .ok_or(ConstructumServerError::NoRepoFound)?;

    let results = crate::server::api::job::db::list_jobs_for_repo(repo_id, state.postgres()).await?;
    Ok(Json(results))
}