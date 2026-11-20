use rand::rngs::OsRng;
use rand::RngCore;

use crate::net::client::{Reply, Site};
use crate::net::gate::Gate;
use crate::support::consts::app::{EMAIL_RANDOM_LEN, PASSWORD_MIN_LEN};
use crate::support::consts::net::{BROWSER_AGENT, ROUTER_TREE, SIGNUP_ACTION, SIGNUP_PAGE};
use crate::support::error::{refusal, AppError, AppResult};
use crate::wire::form::Form;
use crate::wire::shape::rsc_refusal;

pub struct Identity {
    pub fingerprint: String,
    pub password: String,
}

pub fn fresh_identity() -> Identity {
    Identity {
        fingerprint: uuid(),
        password: format!("TH-{}-{}!", hex(4), hex(4)),
    }
}

pub async fn submit(
    site: &Site,
    gate: &Gate,
    email: &str,
    timezone: &str,
    identity: &Identity,
) -> AppResult<()> {
    let token = gate.token().await?;
    if token.chars().count() < PASSWORD_MIN_LEN {
        return Err(AppError::network("the solver returned a short token"));
    }

    let mut form = Form::new();
    form.field("_1_device_fingerprint", &identity.fingerprint)
        .field("_1_timezone", timezone)
        .field("_1_age_confirmed", "1")
        .field("_1_cf-turnstile-response", &token)
        .field("_1_next", "")
        .field("_1_email", email)
        .field("_1_password", &identity.password)
        .field("_1_invite_code", "")
        .field("0", "[\"$undefined\",\"$K1\"]");
    let (content_type, body) = form.finish();

    let extra: Vec<(&str, String)> = vec![
        ("next-action", SIGNUP_ACTION.to_string()),
        ("next-router-state-tree", ROUTER_TREE.to_string()),
        ("origin", crate::support::consts::net::BASE_URL.to_string()),
        (
            "referer",
            format!("{}{}", crate::support::consts::net::BASE_URL, SIGNUP_PAGE),
        ),
        ("user-agent", BROWSER_AGENT.to_string()),
    ];

    let reply = site
        .post_rsc(SIGNUP_PAGE, &content_type, body, &extra)
        .await?;
    judge(&reply)
}

fn judge(reply: &Reply) -> AppResult<()> {
    if reply.status == 400 {
        return Err(AppError::api("the server action envelope was refused"));
    }
    if let Some(text) = rsc_refusal(&reply.text) {
        return Err(AppError::api(refusal(&text)));
    }
    if reply.status >= 500 {
        return Err(AppError::api(
            "the server action threw while creating the account",
        ));
    }
    Ok(())
}

pub fn hex(len: usize) -> String {
    let mut raw = vec![0u8; len];
    OsRng.fill_bytes(&mut raw);
    raw.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn uuid() -> String {
    let mut raw = [0u8; 16];
    OsRng.fill_bytes(&mut raw);
    raw[6] = (raw[6] & 0x0f) | 0x40;
    raw[8] = (raw[8] & 0x3f) | 0x80;
    let body: String = raw.iter().map(|byte| format!("{:02x}", byte)).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &body[0..8],
        &body[8..12],
        &body[12..16],
        &body[16..20],
        &body[20..32]
    )
}

pub fn fresh_email(domain: &str) -> String {
    format!(
        "{}{}@{}",
        crate::support::consts::app::EMAIL_PREFIX,
        hex(EMAIL_RANDOM_LEN / 2),
        domain
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_uuid_has_the_expected_shape() {
        let text = uuid();
        let parts: Vec<&str> = text.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[2].len(), 4);
        assert!(parts[2].starts_with('4'));
        assert!(text.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
        assert_ne!(uuid(), uuid());
    }

    #[test]
    fn the_password_clears_the_minimum_length() {
        let identity = fresh_identity();
        assert!(identity.password.chars().count() >= PASSWORD_MIN_LEN);
        assert!(identity.password.ends_with('!'));
    }

    #[test]
    fn the_email_carries_the_prefix_and_the_domain() {
        let email = fresh_email("example.test");
        assert!(email.starts_with("th-"));
        assert!(email.ends_with("@example.test"));
    }

    #[test]
    fn a_captcha_refusal_is_read_as_a_refusal() {
        let reply = Reply {
            status: 200,
            text: "1:{\"error\":\"Bot check failed. Please refresh and try again.\",\"needCaptcha\":true}"
                .to_string(),
            location: None,
            headers: Vec::new(),
        };
        let outcome = judge(&reply);
        assert!(outcome.is_err());
    }

    #[test]
    fn a_successful_payload_passes() {
        let reply = Reply {
            status: 200,
            text: "0:[\"$\",\"$L7\",null,{\"chunk\":\"error.js\"}]".to_string(),
            location: None,
            headers: Vec::new(),
        };
        assert!(judge(&reply).is_ok());
    }
}
