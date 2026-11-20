use serde_json::json;

use crate::net::client::Site;
use crate::support::consts::net::KEYS_PAGE;
use crate::support::error::{AppError, AppResult};
use crate::wire::shape::plaintext_key;

pub async fn create(site: &Site, label: &str) -> AppResult<String> {
    let reply = site
        .post_json(crate::wire::endpoint::NEW_KEY, &json!({ "label": label }))
        .await?;
    if reply.status != 201 {
        return Err(AppError::api(format!(
            "the key was not created, the server answered HTTP {}",
            reply.status
        )));
    }
    match plaintext_key(&reply.text) {
        Some(key) => Ok(key),
        None => Err(AppError::api(
            "the server created a key but did not hand over its plaintext",
        )),
    }
}

#[allow(dead_code)]
pub fn referer() -> String {
    format!("{}{}", crate::support::consts::net::BASE_URL, KEYS_PAGE)
}
