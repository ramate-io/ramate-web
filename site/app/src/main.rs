use ramate_web_app::config::RamateWebConfig;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = RamateWebConfig::default();
    let app = config.build().await?;
    app.run().await?;
    Ok(())
}