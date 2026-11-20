use serde_json::json;

use crate::net::client::Site;
use crate::support::consts::net::DASHBOARD_PAGE;
use crate::support::error::{AppError, AppResult};

pub async fn enable_free_models(site: &Site) -> AppResult<()> {
    let reply = site
        .post_json(
            crate::wire::endpoint::PRIVACY,
            &json!({
                "free_models_enabled": true,
                "shown_text": "consent-accepted",
                "screen": "consent_modal",
                "path": DASHBOARD_PAGE,
            }),
        )
        .await?;
    if reply.is_success() {
        return Ok(());
    }
    Err(AppError::api(format!(
        "the free models could not be switched on, the server answered HTTP {}",
        reply.status
    )))
}
