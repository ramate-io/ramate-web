use async_trait::async_trait;
use anyhow::{Error};
use tokio::sync::mpsc::{Receiver};

#[async_trait]
pub trait AsyncUserIndex {

    async fn bilink(
        user_id : &str,
        index_id : &str, 
        resource_id : &str
    )->Result<bool, Error>;

    async fn unbilink(
        user_id : &str,
        index_id : &str,
        resource_id : &str
    )->Result<bool, Error>;

    async fn get_index_ids(user_id : &str, resource_id : &str)
    ->Result<Receiver<Result<String, Error>>, Error>;

    async fn get_resource_ids(user_id : &str, index_id : &str)
    ->Result<Receiver<Result<String, Error>>, Error>;

}