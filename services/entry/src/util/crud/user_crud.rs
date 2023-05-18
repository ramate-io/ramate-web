use std::pin::Pin;
use std::future::Future;
use anyhow::{Error};
use super::crud::{AsyncCrud};
use crate::util::{
    privilege::privilege::{AsyncPrivilege},
    indices::index::{AsyncIndices}
};

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

pub const CREATE : &str = "CREATE";
pub const READ : &str = "READ";
pub const UPDATE : &str = "UPDATE";
pub const DELTE : &str = "DELETE";

impl <T, I> AsyncUserCrud<T> for I
    where
        I : AsyncCrud<T> + AsyncPrivilege + AsyncIndices 
{

    fn create_my(user_id : &str, value: T) 
        -> Pin<Box<dyn Future<Output = Result<T, Error>>>> {
        
        Box::pin(async move {

            let val = Self::create(value).await?;
            Self::

        })

    }

}