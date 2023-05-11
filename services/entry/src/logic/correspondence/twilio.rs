use std::{env, sync::{Arc, RwLock}};
use lazy_static::lazy_static;
use twilio_rs::apis::{
    configuration::Configuration,
    default_api::{self as twilio_api, CreateMessageParams}
};

pub trait TwilioConfigGet {

    fn get_config(&self) -> Result<
        Arc<Configuration>, 
        Box<dyn std::error::Error>
    >;

    fn get_default_message_params(&self) -> Result<
        Arc<CreateMessageParams>, 
        Box<dyn std::error::Error>
    >;
    
}

pub struct TwilioSingleton {
    config : RwLock<Option<Arc<Configuration>>>,
    message_params : RwLock<Option<Arc<CreateMessageParams>>>
}

impl TwilioSingleton {
    fn new() -> Self {
        TwilioSingleton {
            config : RwLock::from(None),
            message_params : RwLock::from(None)
        }
    }
}

impl Default for TwilioSingleton {
    fn default() -> Self {
        TwilioSingleton::new()
    }
}

pub static TWILIO_ACCOUNT_ID : &str = "TWILIO_ACCOUNT_ID";
pub static TWILIO_API_KEY : &str = "TWILIO_API_KEY";
pub static TWILIO_AUTH_TOKEN : &str = "TWILIO_AUTH_TOKEN";

impl TwilioConfigGet for TwilioSingleton {

    // get ths config
    fn get_config(&self) -> Result<Arc<twilio_rs::apis::configuration::Configuration>, Box<dyn std::error::Error>> {

        let config_guard = self.config.read().unwrap();
        match config_guard.as_ref() {
            Some(config)=>Ok(config.clone()),
            None=>{
                drop(config_guard); // drop the config_guard so that the write can take place

                let api_key = env::var(TWILIO_API_KEY).expect(
                    format!( "env variable `{}` not present", TWILIO_API_KEY).as_str()
                );

                let api_secret = env::var(TWILIO_AUTH_TOKEN).expect(
                    format!( "env variable `{}` not present", TWILIO_AUTH_TOKEN).as_str()
                );

                let config = Arc::new(twilio_rs::apis::configuration::Configuration {
                    basic_auth : Some((api_key, Some(api_secret))),
                   ..Default::default()
                });
                self.config.write().unwrap().replace(config.clone());
                Ok(config)
            }
        }

    }

    fn get_default_message_params(&self) -> Result<
            Arc<CreateMessageParams>, 
            Box<dyn std::error::Error>
    > {
        
        
        let message_params_guard = self.message_params.read().unwrap();
        match message_params_guard.as_ref() {
            Some(message_params)=>Ok(message_params.clone()),
            None=>{
                drop(message_params_guard); // drop the message_params_guard so that the write can take place

                let account_sid = env::var(TWILIO_ACCOUNT_ID).expect(
                    format!( "env variable `{}` not present", TWILIO_ACCOUNT_ID).as_str()
                );

                let message_params = Arc::new(CreateMessageParams { 
                    account_sid,
                    ..Default::default()
                });
                self.message_params.write().unwrap().replace(message_params.clone());
                Ok(message_params)

            }

        }

    }
}

lazy_static!{
    pub static ref CORRESPONDENCE_TWILIO: TwilioSingleton = TwilioSingleton::new();
}