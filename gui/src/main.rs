
mod gui;
mod rust;
use idf_env_core;

use crate::gui::webview::open_url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn app() -> Result<()> {
    open_url("http://localhost:8000");
    Ok(())
}


#[tokio::main]
async fn main() {
    app().await.unwrap();
}