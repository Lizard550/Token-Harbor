use std::path::Path;

use crate::net::client::Site;
use crate::net::egress::Proxy;
use crate::net::gate::{read_settings, Gate};
use crate::support::consts::net::{BASE_URL, SOLVER_FILE};
use crate::support::error::{AppError, AppResult};
use crate::support::log::format::{gap, lg, ly};

pub async fn execute(root: &Path) -> AppResult<()> {
    ly(&format!("Talking to {}", BASE_URL));

    let site = Site::open(BASE_URL, None)?;
    let reply = site.get(crate::wire::endpoint::MODELS).await?;
    if reply.is_success() {
        lg("The model list answers without a key");
    } else {
        ly(&format!(
            "The model list answers HTTP {} without a key",
            reply.status
        ));
    }

    let page = site.get(crate::support::consts::net::SIGNUP_PAGE).await?;
    if page.is_success() {
        lg("The signup page answers");
    } else {
        ly(&format!("The signup page answers HTTP {}", page.status));
    }

    let path = root.join(SOLVER_FILE);
    match std::fs::read_to_string(&path) {
        Ok(text) => {
            let settings = read_settings(&text);
            if settings.url.is_empty() {
                ly(&format!("{} carries no solver url", SOLVER_FILE));
            } else {
                match Gate::open(&settings.url).await {
                    Ok(_) => lg("The solver accepted the handshake"),
                    Err(err) => ly(&format!("The solver refused the handshake: {}", err)),
                }
            }
        }
        Err(_) => ly(&format!("{} is missing", SOLVER_FILE)),
    }

    for (index, proxy) in read_proxy_lines(root).iter().enumerate() {
        match proxy {
            Ok(proxy) => match egress(proxy).await {
                Ok(country) => lg(&format!("Proxy {} leaves from {}", index + 1, country)),
                Err(err) => ly(&format!("Proxy {} failed: {}", index + 1, err)),
            },
            Err(err) => ly(&format!("Proxy {} is not usable: {}", index + 1, err)),
        }
    }

    gap();
    lg("The server answered every probe");
    Ok(())
}

fn read_proxy_lines(root: &Path) -> Vec<AppResult<Proxy>> {
    let path = root.join(crate::support::consts::app::PROXY_FILE);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(Proxy::parse)
        .collect()
}

async fn egress(proxy: &Proxy) -> AppResult<String> {
    let site = Site::open("https://ipinfo.io", Some(proxy))?;
    let reply = site.get("/json").await?;
    if !reply.is_success() {
        return Err(AppError::network(format!("HTTP {}", reply.status)));
    }
    let value: serde_json::Value = serde_json::from_str(&reply.text)
        .map_err(|_| AppError::network("the answer was not readable"))?;
    let country = value
        .get("country")
        .and_then(|row| row.as_str())
        .unwrap_or("an unknown country");
    Ok(country.to_string())
}
