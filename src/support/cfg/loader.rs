use std::path::{Path, PathBuf};

use crate::net::egress::Proxy;
use crate::support::cfg::schema::Config;
use crate::support::consts::app::{CONFIG_FILE, DATA_FILE, PROXY_FILE};
use crate::support::error::{AppError, AppResult};

pub struct Setup {
    pub root: PathBuf,
    pub config: Config,
    pub keys: Vec<String>,
    pub proxies: Vec<Proxy>,
}

impl Setup {
    pub fn proxy_for(&self, index: usize) -> Option<&Proxy> {
        if self.proxies.is_empty() {
            return None;
        }
        self.proxies.get(index % self.proxies.len())
    }
}

pub fn load(root: &Path) -> AppResult<Setup> {
    let config = Config::load(&root.join(CONFIG_FILE))?;
    let keys = read_keys(&root.join(DATA_FILE))?;
    let proxies = read_proxies(&root.join(PROXY_FILE))?;

    Ok(Setup {
        root: root.to_path_buf(),
        config,
        keys,
        proxies,
    })
}

pub fn read_keys(path: &Path) -> AppResult<Vec<String>> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => {
            return Err(AppError::config(format!(
                "{} could not be read: {}",
                DATA_FILE, err
            )))
        }
    };

    let mut keys: Vec<String> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !keys.iter().any(|known| known == trimmed) {
            keys.push(trimmed.to_string());
        }
    }
    Ok(keys)
}

pub fn read_proxies(path: &Path) -> AppResult<Vec<Proxy>> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(AppError::config(format!(
                "{} could not be read: {}",
                PROXY_FILE, err
            )))
        }
    };

    let mut proxies: Vec<Proxy> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let proxy = Proxy::parse(trimmed).map_err(|err| {
            AppError::config(format!("{} line {}: {}", PROXY_FILE, index + 1, err))
        })?;
        proxies.push(proxy);
    }
    Ok(proxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("th-loader-{}-{}", std::process::id(), name));
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn a_missing_data_file_is_an_empty_list() {
        let dir = scratch("missing");
        let keys = read_keys(&dir.join("nothing.txt")).unwrap();
        assert!(keys.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn comments_and_blanks_are_skipped_and_repeats_dropped() {
        let dir = scratch("keys");
        let path = dir.join("data.txt");
        std::fs::write(
            &path,
            "# one key per line\nthk_live_a\n\nthk_live_b\nthk_live_a\n",
        )
        .unwrap();
        let keys = read_keys(&path).unwrap();
        assert_eq!(keys, vec!["thk_live_a", "thk_live_b"]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_empty_proxy_file_means_direct() {
        let dir = scratch("proxies");
        let path = dir.join("proxy.txt");
        std::fs::write(&path, "# one proxy per line\n").unwrap();
        assert!(read_proxies(&path).unwrap().is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn proxy_is_paired_by_position() {
        let setup = Setup {
            root: PathBuf::from("."),
            config: Config::defaults(),
            keys: Vec::new(),
            proxies: vec![
                Proxy::parse("socks5://127.0.0.1:1080").unwrap(),
                Proxy::parse("socks5://127.0.0.1:1081").unwrap(),
                Proxy::parse("socks5://127.0.0.1:1082").unwrap(),
            ],
        };
        assert_eq!(setup.proxy_for(0).map(|p| p.port), Some(1080));
        assert_eq!(setup.proxy_for(1).map(|p| p.port), Some(1081));
        assert_eq!(setup.proxy_for(2).map(|p| p.port), Some(1082));
        assert_eq!(setup.proxy_for(3).map(|p| p.port), Some(1080));
        assert!(Setup {
            root: PathBuf::from("."),
            config: Config::defaults(),
            keys: Vec::new(),
            proxies: Vec::new(),
        }
        .proxy_for(0)
        .is_none());
    }
}
