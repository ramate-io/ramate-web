use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver};
use anyhow::{Error};

#[async_trait]
pub trait Acls {

    async fn add(
        resource_id: &str, 
        mut ids: Receiver<String>
    ) -> Result<bool, Error>;

    async fn contains(
        resource_id : &str, 
        id : &str
    ) -> Result<bool, Error>;

    // right now we're computing this in the server program
    // an alternative would be take the ids here as sets and intersect them in the redis space
    // however, this doesn't end up being all that useful because one way or another we wil have to compute the 
    // the complete iterator of ids using recursion in program space
    async fn intersects(
        resource_id: &str,
        mut ids: Receiver<String>,
    ) -> Result<bool, Error>;

    async fn empty(resource_id : &str) 
    -> Result<bool,  Error>;

}