#![allow(dead_code)]
pub const SIGNUP: &str = "/login?mode=signup";
pub const DASHBOARD: &str = "/dashboard";
pub const KEYS: &str = "/dashboard/api-keys";
pub const VERIFY: &str = "/verify-email";
pub const PRIVACY: &str = "/api/me/privacy";
pub const SEND_MAIL: &str = "/api/me/send-verification-email";
pub const NEW_KEY: &str = "/api/keys";
pub const ME: &str = "/api/me";
pub const MODELS: &str = "/v1/models";
pub const CHAT: &str = "/v1/chat/completions";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_path_starts_at_the_root() {
        for path in [
            SIGNUP, DASHBOARD, KEYS, VERIFY, PRIVACY, SEND_MAIL, NEW_KEY, ME, MODELS, CHAT,
        ] {
            assert!(path.starts_with('/'));
            assert!(!path.ends_with('/'));
        }
    }

    #[test]
    fn the_api_paths_sit_where_the_browser_calls_them() {
        assert_eq!(ME, "/api/me");
        assert_eq!(NEW_KEY, "/api/keys");
        assert_eq!(CHAT, "/v1/chat/completions");
    }
}
