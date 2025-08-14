use super::app::RamateWebApp;

// Errors thrown by the [RamateWebConfig].
#[derive(Debug, thiserror::Error)]
pub enum RamateWebConfigError {
	#[error("RamateWebConfig failed to build RamateWebApps: {0}")]
	Build(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The configuration used to build the [RamateWebApp].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RamateWebConfig {
}

impl RamateWebConfig {
    /// Build the [RamateWebApp] with the given configuration.
    pub fn build(&self) -> Result<RamateWebApp, RamateWebConfigError> {
        Ok(RamateWebApp::new())
    }
}