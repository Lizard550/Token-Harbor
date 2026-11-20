use std::path::Path;

use crate::job::onboard;
use crate::support::cfg::loader;
use crate::support::error::AppResult;
use crate::support::log::format::{countdown, gap, lg};

pub async fn run(root: &Path, target: usize) -> AppResult<()> {
    cycle(root, target, false).await
}

pub async fn enroll(root: &Path, target: usize) -> AppResult<()> {
    cycle(root, target, true).await
}

async fn cycle(root: &Path, target: usize, fresh: bool) -> AppResult<()> {
    let setup = loader::load(root)?;
    let target = target.max(1);
    let mut index = 0usize;

    while index < target {
        lg(&format!("Starting automation cycle number {}", index + 1));

        if fresh {
            onboard::signup(&setup, index).await?;
        } else {
            match onboard::stored_key(&setup).ok() {
                Some(key) => {
                    let account = onboard::Account {
                        email: onboard::stored_email(&setup)?.unwrap_or_default(),
                        password: String::new(),
                        api_key: key,
                    };
                    onboard::drive(&setup, &account, index).await?;
                }
                None => {
                    onboard::signup(&setup, index).await?;
                }
            }
        }

        index += 1;
        lg(&format!("Automation cycle number {} is complete", index));
        gap();
    }

    countdown(setup.config.sleep_seconds(), "Next cycle starts in").await;
    Ok(())
}
