use crate::net::client::Site;
use crate::support::error::{AppError, AppResult};

#[allow(dead_code)]
pub async fn send(site: &Site) -> AppResult<()> {
    let reply = site
        .post_json(crate::wire::endpoint::SEND_MAIL, &serde_json::json!({}))
        .await?;
    if reply.is_success() {
        return Ok(());
    }
    Err(AppError::api(format!(
        "the verification mail was not queued, the server answered HTTP {}",
        reply.status
    )))
}

pub async fn follow(site: &Site, token: &str) -> AppResult<()> {
    let path = format!("/verify-email?token={}", token);
    let reply = site.get(&path).await?;
    if reply.is_success() || reply.status == 307 || reply.status == 308 {
        return Ok(());
    }
    Err(AppError::api(format!(
        "the mailbox could not be verified, the server answered HTTP {}",
        reply.status
    )))
}

pub async fn confirm(site: &Site) -> AppResult<bool> {
    let reply = site.get("/api/me").await?;
    if !reply.is_success() {
        return Ok(false);
    }
    let value: serde_json::Value = match serde_json::from_str(&reply.text) {
        Ok(value) => value,
        Err(_) => return Ok(false),
    };
    for pointer in ["/email_verified", "/emailVerified", "/data/email_verified"] {
        if let Some(flag) = value.pointer(pointer).and_then(serde_json::Value::as_bool) {
            return Ok(flag);
        }
    }
    Ok(false)
}
