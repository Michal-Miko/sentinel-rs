#![feature(lint_reasons)]

mod repo;
mod util;

use color_eyre::Result;
use tracing::{debug, info, trace};

use repo::{LocalRepository, Repository};
use util::{setup_tracing, LogConfig};

const EXPORT_PATH: &str = "export.xlsx";
const REGISTRY_JSON_PATH: &str = "registry.json";

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing(LogConfig::default())?;
    trace!("Starting Sentinel.");
    debug!("Export path: {EXPORT_PATH}, Registry JSON path: {REGISTRY_JSON_PATH}");

    let local_repo = LocalRepository::new("/mnt/d/dev/rust/sentinel-rs/")?;
    let file_contents = local_repo.fetch_file_contents("README.md")?;
    info!("Got file contents:\n{file_contents}");

    trace!("Exiting Sentinel.");
    Ok(())
}
