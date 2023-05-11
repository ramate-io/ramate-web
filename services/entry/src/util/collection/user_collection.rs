use std::pin::Pin;
use std::future::Future;
use async_trait::async_trait;
use tokio_stream::{Stream};
use anyhow::{Error};
use tokio::sync::mpsc::{Receiver};



pub trait UserCollection<T> {
    fn get_mine(user_id: &str, cursor_id: &str)
    -> Pin<Box<dyn 
        Future<Output = Result<
            Receiver<Result<T, Error>>,
            Error
        >>
    >>;
}