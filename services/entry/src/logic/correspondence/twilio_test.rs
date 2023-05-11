#[cfg(test)]
mod twilio_tests {

    extern crate dotenv;

    use super::super::twilio::{CORRESPONDENCE_TWILIO, TWILIO_API_KEY, TWILIO_AUTH_TOKEN, TwilioConfigGet};
    use std::env;
    use bytes::Buf;
    use dotenv::dotenv;
    use twilio_rs::apis::{
        configuration::Configuration,
        default_api::{self as twilio_api, CreateMessageParams}
    };

    #[test]
    fn gets_config() ->  Result<(), Box<dyn std::error::Error>> {

        env::set_var(TWILIO_API_KEY, "test");
        env::set_var(TWILIO_AUTH_TOKEN, "test");

        CORRESPONDENCE_TWILIO.get_config()?;
        CORRESPONDENCE_TWILIO.get_config()?;

        env::remove_var(TWILIO_API_KEY);
        env::remove_var(TWILIO_AUTH_TOKEN);

        Ok(())
    }

    #[tokio::test]
    async fn sends_message() ->  Result<(), Box<dyn std::error::Error>> {

        dotenv().ok();

        twilio_api::create_message(
            CORRESPONDENCE_TWILIO.get_config()?.as_ref(),
            CreateMessageParams {
                from : Some("+16509772834".to_string()),
                to : "+15302282394".to_string(),
                account_sid : CORRESPONDENCE_TWILIO.get_default_message_params()?.account_sid.clone(),
                body : Some("hello".to_string()),
                ..Default::default()
            }
        ).await?;
        

        Ok(())
    }

}