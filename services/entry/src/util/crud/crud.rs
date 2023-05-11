use std::pin::Pin;
use std::future::Future;
use anyhow::{Error};

pub trait AsyncCrud<T> {
    fn create(value: T) 
    -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;

    fn read(id: &str) 
    -> Pin<Box<dyn Future<Output = Result<Option<T>, Error>>>>;

    fn update(
        id: &str,
        value: T,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;

    fn delete(id: &str) 
    -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;
}