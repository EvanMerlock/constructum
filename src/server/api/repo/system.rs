use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use tracing::info;

use crate::{server::{error::ConstructumServerError}, utils::{post_with_auth, delete_with_auth}};

use super::{GiteaRepository, RepoInfo};

#[tracing::instrument(skip(token))]
pub async fn add_constructum_webhook(url: String, repo: GiteaRepository, token: String) -> Result<i32, ConstructumServerError> {

    #[derive(Debug, Serialize)]
    struct CreateWebhookConfig {
        content_type: String,
        url: String,
    }


    #[derive(Debug, Serialize)]
    struct CreateWebhookPayload {
        active: bool,
        //authorization_header: String,
        branch_filter: Option<String>,
        config: CreateWebhookConfig,
        events: Vec<String>,
        #[serde(rename="type")]
        wh_type: String,
    }

    let cwp = CreateWebhookPayload {
        active: true,
        // TODO: configurable
        branch_filter: None,
        config: CreateWebhookConfig { content_type: "json".to_owned(), url: "http://10.16.24.2:3001/api/v1/webhook".to_owned() },
        // TODO: configurable
        events: vec!["push".to_owned()],
        wh_type: "gitea".to_owned(),
    };

    let body = serde_json::to_string(&cwp)?;
    let req_url = format!("{url}/api/v1/repos/{}/{}/hooks", repo.owner.login, repo.name);
    let resp = post_with_auth(req_url, "Authorization", token, body, "application/json").await?;
    info!("resp {:?}", resp);

    #[derive(Debug, Deserialize)]
    struct CreateWebhookResponse {
        id: i32
    }

    let resp_id: CreateWebhookResponse = resp.json().await?;

    Ok(resp_id.id)
}

#[tracing::instrument]
pub async fn remove_constructum_webhook(url: String, db_repo: RepoInfo, token: String) -> Result<(), ConstructumServerError> {
    match db_repo.webhook_id {
        Some(wh_id) => {
            let resp = delete_with_auth(format!("{url}/api/v1/repos/{}/{}/hooks/{}", db_repo.repo_owner, db_repo.repo_name, wh_id), "Authorization", token).await?;

            if resp.status() != StatusCode::OK {
                // TODO: return error
            }
        
            Ok(())
        },
        None => {
            Ok(())
        }
    }

}