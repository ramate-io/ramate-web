use async_trait::async_trait;
use anyhow::{Error};
use tokio::sync::mpsc::{Receiver};

#[async_trait]
pub trait UserExpander {

    async fn expand_user(
        user_id : &str
    )->Result<Receiver<Result<String, Error>>, Error>;

}