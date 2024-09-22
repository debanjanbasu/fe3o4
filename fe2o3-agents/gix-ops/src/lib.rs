generate!({ generate_all });

use anyhow::{Ok, Result};
use wasi::logging::logging::{log, Level};
use wit_bindgen::generate;

struct GixOps;

impl GixOps {
    async fn load_git(repo: &str) -> Result<()> {
        let body = reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?;
        log(Level::Info, "gix-ops", &body);
        Ok(())
    }
}
