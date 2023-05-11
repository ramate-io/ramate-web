use std::pin::Pin;
use std::future::Future;
use async_trait::async_trait;
use futures_core::stream::Stream;
use tokio::sync::mpsc::{Receiver};
use anyhow::{Error};

pub trait AsyncNamedCollection {
    fn get_collection_name() -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
}

pub trait AsyncCollection<T> {

    fn get_all(cursor_id: &str)
    -> Pin<Box<dyn 
        Future<Output = Result<
            Receiver<Result<T, Error>>,
            Error
        >>
    >>;

}

pub trait AsyncCollectionAdmin {

    fn clear()
    -> Pin<Box<dyn 
        Future<Output = Result<
            bool,
            Error
        >>
    >>;

}