use serde_json::Value;

pub fn rsc_refusal(body: &str) -> Option<String> {
    for line in body.lines() {
        let colon = match line.find(':') {
            Some(index) => index,
            None => continue,
        };
        let payload = &line[colon + 1..];
        let value: Value = match serde_json::from_str(payload) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let object = match value.as_object() {
            Some(object) => object,
            None => continue,
        };
        if let Some(text) = object.get("error").and_then(Value::as_str) {
            if !text.trim().is_empty() {
                return Some(text.trim().to_string());
            }
        }
        if object.get("needCaptcha").and_then(Value::as_bool) == Some(true) {
            return Some("the human check was refused".to_string());
        }
    }
    None
}

pub fn verify_token(blob: &str) -> Option<String> {
    let marker = "verify-email?token=";
    let start = blob.find(marker)? + marker.len();
    let rest = &blob[start..];
    let token: String = rest
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '='))
        .collect();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

pub fn plaintext_key(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    for pointer in ["/plaintext", "/key", "/data/plaintext", "/data/key"] {
        if let Some(text) = value.pointer(pointer).and_then(Value::as_str) {
            if !text.trim().is_empty() {
                return Some(text.trim().to_string());
            }
        }
    }
    None
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a [Value] {
    if let Some(list) = value.get(key).and_then(Value::as_array) {
        return list;
    }
    if let Some(list) = value.as_array() {
        return list;
    }
    &[]
}

pub fn message_ids(body: &str) -> Vec<String> {
    let value: Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    rows(&value, "hydra:member")
        .iter()
        .filter_map(|row| row.get("id").and_then(Value::as_str))
        .map(|id| id.to_string())
        .collect()
}

pub fn domain_list(body: &str) -> Vec<String> {
    let value: Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    rows(&value, "hydra:member")
        .iter()
        .filter(|row| row.get("isActive").and_then(Value::as_bool).unwrap_or(true))
        .filter_map(|row| row.get("domain").and_then(Value::as_str))
        .map(|domain| domain.to_string())
        .collect()
}

pub fn chat_text(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    let content = value.pointer("/choices/0/message/content")?.as_str()?;
    Some(content.trim().to_string())
}

fn joined(row: &Value, keys: &[&str]) -> String {
    let mut out = String::new();
    for key in keys {
        if let Some(text) = row.get(*key).and_then(Value::as_str) {
            out.push_str(text);
            out.push('\n');
        }
    }
    out
}

pub fn mail_io_bodies(body: &str) -> Vec<String> {
    let value: Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let rows = match value.as_array() {
        Some(rows) => rows,
        None => return Vec::new(),
    };
    rows.iter()
        .map(|row| joined(row, &["body_text", "body_html", "subject"]))
        .filter(|blob| !blob.trim().is_empty())
        .collect()
}

pub fn mail_lol_bodies(body: &str) -> Vec<String> {
    let value: Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let rows = match value.get("emails").and_then(Value::as_array) {
        Some(rows) => rows,
        None => return Vec::new(),
    };
    rows.iter()
        .map(|row| joined(row, &["body", "body_html", "subject"]))
        .filter(|blob| !blob.trim().is_empty())
        .collect()
}

pub fn guerrilla_ids(body: &str) -> Vec<String> {
    let value: Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let rows = match value.get("list").and_then(Value::as_array) {
        Some(rows) => rows,
        None => return Vec::new(),
    };
    rows.iter()
        .filter_map(|row| row.get("mail_id"))
        .filter_map(|id| match id {
            Value::String(text) => Some(text.clone()),
            Value::Number(number) => Some(number.to_string()),
            _ => None,
        })
        .collect()
}

pub fn guerrilla_body(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    let text = value.get("mail_body")?.as_str()?.trim().to_string();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

pub fn model_ids(body: &str) -> Vec<String> {
    let value: Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let rows = match value.get("data").and_then(Value::as_array) {
        Some(rows) => rows,
        None => return Vec::new(),
    };
    rows.iter()
        .filter_map(|row| row.get("id").and_then(Value::as_str))
        .filter(|id| !id.trim().is_empty())
        .map(|id| id.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_successful_rsc_payload_is_not_read_as_an_error() {
        let body = "0:[\"$\",\"$L7\",null,{\"chunk\":\"error-boundary.js\"}]\n1:{\"ok\":true}";
        assert!(rsc_refusal(body).is_none());
    }

    #[test]
    fn the_captcha_refusal_is_found_on_its_own_line() {
        let body = "1:{\"error\":\"Bot check failed. Please refresh and try again.\",\"needCaptcha\":true}";
        let text = rsc_refusal(body).unwrap();
        assert!(text.starts_with("Bot check failed"));
    }

    #[test]
    fn the_verify_token_is_taken_from_a_mail_blob() {
        let blob =
            "{\"text\":\"open https://tokenharbor.ai/verify-email?token=abc123_XY-z.9= now\"}";
        assert_eq!(verify_token(blob).as_deref(), Some("abc123_XY-z.9="));
        assert!(verify_token("nothing here").is_none());
    }

    #[test]
    fn the_key_is_read_from_the_plaintext_field() {
        let body = r#"{"plaintext":"thk_live_abcdef","key":{"id":"1","prefix":"thk_live_"}}"#;
        assert_eq!(plaintext_key(body).as_deref(), Some("thk_live_abcdef"));
        assert!(plaintext_key(r#"{"key":{"id":"1"}}"#).is_none());
    }

    #[test]
    fn the_domain_list_skips_inactive_rows() {
        let body = r#"{"hydra:member":[{"domain":"a.test","isActive":true},{"domain":"b.test","isActive":false}]}"#;
        assert_eq!(domain_list(body), vec!["a.test"]);
    }

    #[test]
    fn the_domain_list_also_reads_a_plain_array() {
        let body = r#"[{"domain":"euploading.com","isActive":true}]"#;
        assert_eq!(domain_list(body), vec!["euploading.com"]);
    }

    #[test]
    fn the_message_list_also_reads_a_plain_array() {
        let body = r#"[{"id":"abc"},{"id":"def"}]"#;
        assert_eq!(message_ids(body), vec!["abc", "def"]);
    }

    #[test]
    fn the_model_list_drops_the_empty_ids() {
        let body =
            r#"{"data":[{"id":"mimo-v2.5:free"},{"id":""},{"id":"deepseek-v4-flash:free"}]}"#;
        assert_eq!(
            model_ids(body),
            vec!["mimo-v2.5:free", "deepseek-v4-flash:free"]
        );
    }

    #[test]
    fn the_chat_text_is_the_first_choice() {
        let body = r#"{"choices":[{"message":{"role":"assistant","content":" ok "}}]}"#;
        assert_eq!(chat_text(body).as_deref(), Some("ok"));
    }
}
