use std::pin::Pin;
use std::future::Future;
use anyhow::{Error};

pub trait AsyncUserCrud<T> {
    fn create_my(user_id : &str, value: T) 
    -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;

    fn read_my(user_id : &str, id: &str) 
    -> Pin<Box<dyn Future<Output = Result<Option<T>, Error>>>>;

    fn update_my(
        user_id : &str,
        id : &str,
        value : T,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;

    fn delete_my(
        user_id : &str,
        id : &str
    ) -> Pin<Box<dyn Future<Output = Result<T, Error>>>>;
}