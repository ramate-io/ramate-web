use async_trait::async_trait;
use anyhow::Error;

#[async_trait]
pub trait Id : Sized {

    async fn generate_id()
    ->Result<String, Error>;
    
    async fn get_id(&self)
    ->Result<String, Error>;

    // Should produce a new copy of struct with 
    // the appropriate id
    async fn with_id(&self, id : &str)
    ->Result<Self, Error>;

}