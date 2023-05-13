use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver};
use anyhow::{Error};

#[async_trait]
pub trait Acls : Send + Sync {

    async fn add<'a>(
        &self, key: &str, 
        mut ids: Receiver<&'a str>
    ) -> Result<&Self, Error>;

    async fn contains(
        &self, key : &str, 
        id : &str
    ) -> Result<bool, Error>;

    // right now we're computing this in the server program
    // an alternative would be take the ids here as sets and intersect them in the redis space
    // however, this doesn't end up being all that useful because one way or another we wil have to compute the 
    // the complete iterator of ids using recursion in program space
    async fn intersects<'a>(
        &self,
        key: &str,
        mut ids: Receiver<&'a str>,
    ) -> Result<bool, Error>;

    async fn empty(&self, key : &str) 
    -> Result<&Self,  Error>;

}