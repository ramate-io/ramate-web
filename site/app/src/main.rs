use ramate_web_app::config::RamateWebConfig;

pub fn main() -> Result<(), anyhow::Error> {
    let config = RamateWebConfig::default();
    let app = config.build()?;
    app.run()?;
    Ok(())
}