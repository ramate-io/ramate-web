use async_trait::async_trait;
use anyhow::{anyhow, Error};
use tokio::sync::mpsc::{Receiver};
use std::sync::{Arc};

pub const root_partition : &str = "root";

#[async_trait]
pub trait AsyncIndexer {

    async fn link(
        index_id : &str, 
        resource_id : &str
    )->Result<bool, Error>;

    async fn unlink(
        index_id : &str, 
        resource_id : &str
    )->Result<bool, Error>;

    async fn get_index_ids(
        resource_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;

    async fn get_resource_ids(
        index_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;
}

#[async_trait]
pub trait AsyncIndex : Send + Sync {

    // links or unlinks the resource to the index
    // returns bool representing whether or the resource was linked to the index
    async fn manage_link(
        &self,
        partition_id : &str, // this is useful if you to deploy the same logic across several indices, otherwise, you can just use root index
        resource_id : &str
    )->Result<bool, Error>;

}

#[async_trait]
pub trait AsyncIndices {


    async fn get_indices(

    )->Result<Receiver<Result<(String, Box<dyn AsyncIndex>), Error>>, Error>;

    async fn get_index(
        index_id : &str
    )->Result<Box<dyn AsyncIndex>, Error>;

    async fn add_index(
        index : Box<dyn AsyncIndex>
    )->Result<bool, Error>;

    async fn remove_index(
        index : &str
    )->Result<Box<dyn AsyncIndex>, Error>;

      // links or unlinks for all indices
    // returns receiver of tuples of index id and bool representing whether the link was successful
    async fn manage_links(
        &self, 
        partition_id : &str,
        resource_id: &str
    ) -> Result<Receiver<Result<(String, bool), Error>>, Error> {
        let mut indx = Self::get_indices().await?;
        let (result_sender, result_receiver) = tokio::sync::mpsc::channel(10);
        let mut handles = Vec::new();
        while let Some(res) = indx.recv().await {
            let (key, index) = res?;
            let index = Arc::new(index);
            let sender = result_sender.clone();
            let ptid = Arc::new(partition_id.to_string());
            let rid = Arc::new(resource_id.to_string());
            let handle = tokio::spawn(async move {
                let link_result = index.clone().manage_link(
                    &ptid.clone(), 
                    &rid.clone()
                ).await?;
                let _ = sender.send(Ok((key, link_result))).await;
                Ok::<(), Error>(())
            });
            handles.push(handle);
        }

        Ok(result_receiver)
    }


}