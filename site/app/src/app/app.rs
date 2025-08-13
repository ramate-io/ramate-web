pub mod dioxus_app;

/// Errors thrown by the [RamateWebApp]..
#[derive(Debug, thiserror::Error)]
pub enum RamateWebAppError {
	#[error("RamateWebApp runtime error: {0}")]
	Runtime(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// The application instance.
pub struct RamateWebApp;

impl RamateWebApp {
    pub fn new() -> Self {
        Self
    }

    /// Run the application.
    /// 
    /// This is asynchronous and fallible because we might add to Dioxus other runtimes. 
    pub async fn run(&self) -> Result<(), RamateWebAppError> {
        dioxus::launch(dioxus_app::DioxusApp);
        Ok(())
    }
}