use async_trait::async_trait;
use anyhow::{Error};
use tokio::sync::mpsc::{Receiver};

#[async_trait]
pub trait AsyncIndex {

    async fn manage_link(
        &self,
        index_id : &str, 
        resource_id : &str
    )->Result<bool, Error>;

    async fn get_index_ids(
        &self,
        resource_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;

    async fn get_resource_ids(
        &self,
        index_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;

}

#[async_trait]
pub trait AsyncIndices {

    async fn manage_links(
        resource_id : &str
    )->Result<String, Error>;

    async fn get_index_ids(
        &self,
        resource_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;

    async fn get_resource_ids(
        &self,
        index_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;

}