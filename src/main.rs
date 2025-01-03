use railway_cron::fetch_urls;
use tracing::Level;

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    fetch_urls()?;

    Ok(())
}
