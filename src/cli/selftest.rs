use std::sync::atomic::{AtomicUsize, Ordering};

use crate::net::egress::Proxy;
use crate::step::signup::{fresh_email, fresh_identity, uuid};
use crate::support::error::{AppError, AppResult};
use crate::support::log::format::{clean_text, clock, gap, lg, ly, mask_email, mask_key, num};
use crate::wire::form::Form;
use crate::wire::shape::{chat_text, model_ids, plaintext_key, rsc_refusal, verify_token};

pub fn execute() -> AppResult<()> {
    ly("The self test runs without touching the network");
    check(uuid().len() == 36, "A fingerprint is 36 characters")?;
    check(
        uuid().split('-').count() == 5,
        "A fingerprint carries five groups",
    )?;

    let identity = fresh_identity();
    check(
        identity.password.chars().count() >= 12,
        "A password clears the minimum length",
    )?;

    let email = fresh_email("example.test");
    check(
        email.starts_with("th-") && email.ends_with("@example.test"),
        "A mailbox address carries the prefix and the domain",
    )?;
    check(
        !mask_email(&email).contains(email.split('@').next().unwrap_or_default()),
        "A mailbox address is masked at the head",
    )?;

    let mut form = Form::new();
    form.field("_1_email", &email)
        .field("0", "[\"$undefined\",\"$K1\"]");
    let (content_type, body) = form.finish();
    let text =
        String::from_utf8(body).map_err(|_| AppError::api("the form body was not readable"))?;
    check(
        content_type.starts_with("multipart/form-data; boundary=----WebKitFormBoundary"),
        "The form carries a browser boundary",
    )?;
    check(
        text.contains("name=\"_1_email\"") && text.contains(&email),
        "The form carries the email field",
    )?;

    check(
        rsc_refusal("1:{\"error\":\"Bot check failed.\",\"needCaptcha\":true}").is_some(),
        "A refusal line is read as a refusal",
    )?;
    check(
        rsc_refusal("0:[\"$\",\"$L7\",null,{\"chunk\":\"error.js\"}]").is_none(),
        "A normal payload is not read as a refusal",
    )?;
    check(
        verify_token("https://tokenharbor.ai/verify-email?token=abc123_XY-z.9= x").as_deref()
            == Some("abc123_XY-z.9="),
        "The verification token is taken from the mail",
    )?;
    check(
        plaintext_key(r#"{"plaintext":"thk_live_abcdef"}"#).as_deref() == Some("thk_live_abcdef"),
        "The key is read from the plaintext field",
    )?;
    check(
        model_ids(r#"{"data":[{"id":"mimo-v2.6-flash:free"}]}"#).len() == 1,
        "A model list is read from the data array",
    )?;
    check(
        chat_text(r#"{"choices":[{"message":{"content":" ok "}}]}"#).as_deref() == Some("ok"),
        "A chat reply is read from the first choice",
    )?;

    let proxy = Proxy::parse("socks5://user:secret@192.0.2.10:10043")?;
    check(
        proxy.masked() == "http://user:pass@192*****10:10043",
        "A proxy is masked before it is printed",
    )?;
    check(
        proxy.url() == "socks5://user:secret@192.0.2.10:10043",
        "A proxy keeps its credentials in the url",
    )?;

    check(
        mask_key("thk_live_abcdefghijklmnop") == "thk_live**ghijklmnop",
        "A key keeps a head and a tail only",
    )?;
    check(
        clean_text("###", "fallback") == "fallback",
        "Empty text falls back",
    )?;
    check(
        clock(3_723) == "01:02:03",
        "A countdown is written as a clock",
    )?;
    check(num(1.23456) == "1.2346", "A number keeps four places")?;

    let passed = PASSED.load(Ordering::Relaxed);
    let failed = FAILED.load(Ordering::Relaxed);

    gap();
    if failed > 0 {
        return Err(AppError::api(format!(
            "{} of {} checks failed",
            failed,
            passed + failed
        )));
    }
    lg(&format!("All {} checks passed", passed));
    Ok(())
}

static PASSED: AtomicUsize = AtomicUsize::new(0);
static FAILED: AtomicUsize = AtomicUsize::new(0);

fn check(flag: bool, msg: &str) -> AppResult<()> {
    if flag {
        PASSED.fetch_add(1, Ordering::Relaxed);
        lg(&format!("The check passed for {}", msg));
        Ok(())
    } else {
        FAILED.fetch_add(1, Ordering::Relaxed);
        Err(AppError::api(msg))
    }
}
