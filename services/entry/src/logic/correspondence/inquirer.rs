use super::super::super::entry::{Contact, SkillTag};

pub struct MediumInquiryConfirmation {
    pub id : String,
    pub which : String
}

#[async_trait::async_trait]
pub trait InquireMedium {

    async fn inquire(&mut self, contacts : Box<Vec<Contact>>, skills : Box<Vec<SkillTag>>)->Result<MediumInquiryConfirmation, Box<dyn std::error::Error>>;

}

