use super::inquirer::{InquireMedium, MediumInquiryConfirmation};
use super::super::entry::{Contact, ContactMedium, SkillTag, Inquiry};
use super::sendgrid::{CorrespondenceSendgrid, SendgridClientGet, SendgridConfigGet, SendgridEmailThreshold};
use rayon::result;
use sendgrid::{Mail, Destination};
use reqwest;
use sendgrid_rs::models::{PostValidationsEmailRequest, PostValidationsEmail200Response};
use std::env;
use sendgrid_rs::apis::{email_address_validation_api};

#[async_trait::async_trait]
trait ValidateEmail {

    async fn email_exists(&mut self, email : &str)->Result<bool, Box<dyn std::error::Error>> {
        Ok(false)
    }

}


pub struct EmailInquirer {
 
}

#[async_trait::async_trait]
impl ValidateEmail for EmailInquirer {
    
    async fn email_exists(&mut self, email : &str)->Result<bool, Box<dyn std::error::Error>> {
        
        let config = CorrespondenceSendgrid.get_config()?;
        let threshold = CorrespondenceSendgrid.get_valid_email_threshold()?;
        let res : PostValidationsEmail200Response = email_address_validation_api::p_ost_validations_email(
            &config, 
            email_address_validation_api::POstValidationsEmailParams { 
                post_validations_email_request: Some(PostValidationsEmailRequest {
                    email : Some(email.into()),
                    source : None
                })
            }
        ).await?;


        match res.result.score {
            None=>Err("Invalid email.".into()),
            Some(score)=>match score.as_f64() {
                None=>Err("Invalid email.".into()),
                Some(fscore)=>Ok(fscore > threshold)
            },
        }

    }

}

#[async_trait::async_trait]
impl InquireMedium for EmailInquirer {

    async fn inquire(&mut self, contacts : Box<Vec<Contact>>, skills : Box<Vec<SkillTag>>)->Result<MediumInquiryConfirmation, Box<dyn std::error::Error>>{

        let mut client = CorrespondenceSendgrid.get_client().unwrap();

        let mail = Mail::new();

        for contact in contacts.iter_mut() {
            mail.add_to(Destination { 
                address: contact.value.as_str(),
                name : "".into()
            });
        }

        client.send(mail);

        unimplemented!("This method is not yet implemented.")

    }

}

impl Default for EmailInquirer {

    fn default() -> Self {
        EmailInquirer {  }
    }

}