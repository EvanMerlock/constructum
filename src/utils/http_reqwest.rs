use axum::http::HeaderValue;
use reqwest::{Response, Body};

#[tracing::instrument(skip(token))]
pub async fn get_with_auth(url: String, header_name: &'static str, token: String) -> Result<Response, reqwest::Error> {
    let mut header_map = reqwest::header::HeaderMap::new();
    let mut a_tok = HeaderValue::from_str(&token).expect("failed to set authorization token");
    a_tok.set_sensitive(true);
    header_map.insert(header_name, a_tok);
    let req_client = reqwest::ClientBuilder::new().default_headers(header_map).build()?;

    let req = req_client.get(url).build()?;
    req_client.execute(req).await
}

#[tracing::instrument(skip(token, body))]
pub async fn post_with_auth<T>(url: String, header_name: &'static str, token: String, body: T, content_type: &'static str) -> Result<Response, reqwest::Error> where T: Into<Body> {
    let mut header_map = reqwest::header::HeaderMap::new();
    let mut a_tok = HeaderValue::from_str(&token).expect("failed to set authorization token");
    a_tok.set_sensitive(true);
    header_map.insert(header_name, a_tok);
    let content_type = HeaderValue::from_str(content_type).expect("failed to set content-type");
    header_map.insert("Content-Type", content_type);
    let req_client = reqwest::ClientBuilder::new().default_headers(header_map).build()?;

    let req = req_client.post(url).body(body).build()?;
    req_client.execute(req).await
}

#[tracing::instrument(skip(token))]
pub async fn delete_with_auth(url: String, header_name: &'static str, token: String) -> Result<Response, reqwest::Error> {
    let mut header_map = reqwest::header::HeaderMap::new();
    let mut a_tok = HeaderValue::from_str(&token).expect("failed to set authorization token");
    a_tok.set_sensitive(true);
    header_map.insert(header_name, a_tok);
    let req_client = reqwest::ClientBuilder::new().default_headers(header_map).build()?;

    let req = req_client.delete(url).build()?;
    req_client.execute(req).await
}