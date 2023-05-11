use async_trait::async_trait;
use anyhow::{Error};

#[async_trait]
pub trait AsyncToken {

    async fn generate_token(user_id : &str)->Result<&str, Error>;

    // Gets the user id from a token
    // Should error if the token is invalid
    async fn get_user_id(token : &str)->Result<&str, Error>;

}