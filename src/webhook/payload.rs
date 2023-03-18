#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitWebhookPayload {
    secret: Option<String>,
    #[serde(rename(deserialize = "ref"))]
    pub git_reference: String,
    before: String,
    pub after: String,
    compare_url: String,
    commits: Vec<CommitWebhookPayload>,
    pub repository: RepositoryWebhookPayload,
    pusher: UserWebhookPayload,
    sender: UserWebhookPayload,

}

#[derive(Debug, Deserialize)]
pub struct CommitWebhookPayload {
    id: String,
    message: String,
    url: String,
    author: CommitUserWebhookPayload,
    committer: CommitUserWebhookPayload,
    timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitUserWebhookPayload {
    name: String,
    email: String,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryWebhookPayload {
    id: u64,
    owner: UserWebhookPayload,
    pub name: String,
    full_name: String,
    description: String,
    private: bool,
    fork: bool,
    pub html_url: String,
    pub ssh_url: String,
    clone_url: String,
    website: String,
    stars_count: u64,
    forks_count: u64,
    watchers_count: u64,
    open_issues_count: u64,
    default_branch: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct UserWebhookPayload {
    id: u64,
    login: String,
    full_name: String,
    email: String,
    avatar_url: String,
    username: String,
}