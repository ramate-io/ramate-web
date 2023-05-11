use sendgrid_rs;
use std::{env, sync::Arc};
use lazy_static::lazy_static;

pub trait SendgridConfigGet {
    fn get_config(&mut self) -> Result<Arc<sendgrid_rs::apis::configuration::Configuration>, Box<dyn std::error::Error>>;
}

pub trait SendgridEmailThreshold {
    fn get_valid_email_threshold(&mut self) -> Result<f64, Box<dyn std::error::Error>>;
}

pub struct SendgridSingleton {
    configuration: Option<Arc::<sendgrid_rs::apis::configuration::Configuration>>,
    valid_email_threshold: Option<f64>,
}

impl SendgridSingleton {
    fn new() -> Self {
        SendgridSingleton {
            configuration: None,
            valid_email_threshold: None,
        }
    }
}

impl Default for SendgridSingleton {
    fn default() -> Self {
        SendgridSingleton::new()
    }
}


impl SendgridConfigGet for SendgridSingleton {
    fn get_config(&mut self) -> Result<Arc<sendgrid_rs::apis::configuration::Configuration>, Box<dyn std::error::Error>> {
        match self.configuration.as_ref() {
            Some(configuration) => Ok(configuration.clone()),
            None => {
                let configuration = Arc::new(sendgrid_rs::apis::configuration::Configuration {
                    bearer_access_token: Some(env::var("SENDGRID_API_KEY")?),
                    ..Default::default()
                });
                self.configuration = Some(configuration.clone());
                Ok(configuration)
            }
        }
    }
}

impl SendgridEmailThreshold for SendgridSingleton {
    fn get_valid_email_threshold(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        match self.valid_email_threshold {
            Some(threshold) => Ok(threshold),
            None => {
                let threshold_str = env::var("SENDGRID_VALID_EMAIL_THRESHOLD")?;
                let threshold: f64 = threshold_str.parse()?;
                self.valid_email_threshold = Some(threshold);
                Ok(threshold)
            }
        }
    }
}



lazy_static! {
    pub static ref CORRESPONDENCE_SENDGRID: SendgridSingleton = SendgridSingleton::new();
}
