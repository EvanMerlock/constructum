use axum::{Json, extract::{State, Path}, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{ConstructumState, server::{error::ConstructumServerError, api::repo::{db::list_repos, GitRepoResponse}}, utils};

use super::{RepoInfo, GiteaRepository, RegisterRepositoryPayload};


pub async fn list_known_repos(
    State(state): State<ConstructumState>
) -> Result<Json<Vec<RepoInfo>>, ConstructumServerError> {

    let repo_info = super::db::list_repos(state.postgres).await?;

    Ok(Json(repo_info))
}

pub async fn list_all_repos(
    headers: HeaderMap,
    State(state): State<ConstructumState>
) -> Result<Json<Vec<GitRepoResponse>>, ConstructumServerError> {
    let auth_tok = headers.get("Authorization").ok_or(ConstructumServerError::BadAuthorization)?;
    let resp = utils::get_with_auth(format!("{}/api/v1/repos/search", state.git_server_url), "Authorization", auth_tok.to_str()?.to_owned()).await?;

    #[derive(Debug, Deserialize)]
    struct GiteaRepositoryResponse {
        data: Vec<GiteaRepository>,
        ok: bool
    }

    let git_repos_resp: GiteaRepositoryResponse = resp.json().await?;

    if !git_repos_resp.ok {
        todo!()
    }
    
    let git_repos = git_repos_resp.data;
    let known_repos = list_repos(state.postgres).await?;

    // TODO: this is O(n^2)
    let git_repos = git_repos.into_iter().map(|x| {
        let known_repo = known_repos.iter().find(|y| y.git_id == x.id);
        GitRepoResponse {
            id: known_repo.map(|x| x.repo_uuid),
            name: x.name,
            description: x.description,
            html_url: x.html_url,
            ssh_url: x.ssh_url,
            owner: x.owner,
            is_registed: known_repo.is_some(),
        }
    }).collect();

    Ok(Json(git_repos))
}

pub async fn get_repo(
    Path(repo_id): Path<Uuid>,
    State(state): State<ConstructumState>
) -> Result<Json<RepoInfo>, ConstructumServerError> {
    let repo_info = super::db::get_repo(repo_id, state.postgres).await?;

    Ok(Json(repo_info))
}

pub async fn register_repository(
    headers: HeaderMap,
    State(state): State<ConstructumState>,
    Json(payload): axum::extract::Json<RegisterRepositoryPayload>
) -> Result<impl IntoResponse, ConstructumServerError> {
    let auth_tok = headers.get("Authorization").ok_or(ConstructumServerError::BadAuthorization)?;
    let corr_auth_tok = auth_tok.to_str()?.to_owned();
    let resp = utils::get_with_auth(format!("{}/api/v1/repos/{}/{}", state.git_server_url, payload.owner, payload.name), "Authorization", corr_auth_tok.clone()).await?;

    let git_repo: GiteaRepository = resp.json().await?;

    // checking for existence
    if super::db::get_repo_by_git_id(git_repo.id, state.postgres.clone()).await?.is_some() {
        return Err(ConstructumServerError::RepoAlreadyRegistered);
    }

    // need to know wh_id before adding repo to DB
    // TODO: considering reordering
    let wh_id = super::system::add_constructum_webhook(state.git_server_url, git_repo.clone(), corr_auth_tok.clone()).await?;

    let repo_uuid = Uuid::new_v4();

    let payload = super::RepoInfo {
        repo_uuid,
        git_id: git_repo.id,
        repo_url: git_repo.html_url.clone(),
        repo_owner: git_repo.owner.login.clone(),
        repo_name: git_repo.name.clone(),
        webhook_id: wh_id,
    };

    super::db::register_repo(state.postgres, payload).await?;

    // TODO: mark this as json
    Ok((StatusCode::OK, format!("{{ uuid: \"{repo_uuid}\" }}")))
}

pub async fn remove_repository(
    headers: HeaderMap,
    Path(repo_id): Path<Uuid>,
    State(state): State<ConstructumState>,
) -> Result<impl IntoResponse, ConstructumServerError> {
    // checking for existence
    let _repo_ref = 
        super::db::get_repo_optional(repo_id, state.postgres.clone())
            .await?
            .ok_or(ConstructumServerError::NoRepoFound)?;

    let auth_tok = headers.get("Authorization").ok_or(ConstructumServerError::BadAuthorization)?;
    let corr_auth_tok = auth_tok.to_str()?.to_owned();

    let repo_info = super::db::get_repo(repo_id, state.postgres.clone()).await?;


    super::system::remove_constructum_webhook(state.git_server_url, repo_info, corr_auth_tok.clone()).await?;
    super::db::delete_repo(repo_id, state.postgres).await?;

    Ok((StatusCode::NO_CONTENT, ""))
}