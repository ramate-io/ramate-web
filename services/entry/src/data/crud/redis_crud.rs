use crate::util::crud::{crud::{AsyncCrud}, id::Id};
use crate::util::redis::redis::RedisConnection;
use crate::util::collection::collection::AsyncNamedCollection;
use prost::Message;
use redis::AsyncCommands;
use std::pin::Pin;
use std::future::Future;
use std::collections::VecDeque;
use anyhow::{anyhow, Error};

pub trait RedisAsyncCrud : Message + Id + AsyncNamedCollection + RedisConnection + Send + Default {

}

impl<T> AsyncCrud<T> for T
where
    T: RedisAsyncCrud + 'static,
{
    fn create(value: T) -> Pin<Box<dyn Future<Output = Result<T, Error>>>> {
        Box::pin(async move {
            let collection = Self::get_collection_name().await?;
            let id = Self::generate_id().await?;
            let created_value = value.with_id(id.as_str()).await?;
            let mut conn = Self::get_connection().await?;
            let field_count: u8 = conn
                .hset(collection, id.as_str(), created_value.encode_to_vec())
                .await?;

            Ok(created_value)

        })
    }

    fn read(id: &str) -> Pin<Box<dyn Future<Output = Result<Option<T>,Error>>>> {

        let id_cpy = id.to_string();
        Box::pin(async move {
            // Implementation for read function
            let collection = Self::get_collection_name().await?;
            let mut conn = Self::get_connection().await?;
            let buf : Option<Vec<u8>> = conn.hget(collection.as_str(), id_cpy.as_str()).await?;

            match buf {
                Some(buf)=> match Self::decode(
                    VecDeque::from(buf)
                ){
                    Ok(res)=>Ok(Some(res)),
                    Err(res)=>Err(anyhow!(
                        "Could not decode message from collection: {}", collection.as_str()).into()
                    )  
                },
                None=>Ok(None)
            }

        })
    }

    fn update(
        id: &str,
        value: T,
    ) -> Pin<Box<dyn Future<Output = Result<T, Error>>>> {

        let id_cpy = id.to_string();
        Box::pin(async move {
            let collection = Self::get_collection_name().await?;
            let true_value = value.with_id(id_cpy.as_str()).await?;
            let mut conn = Self::get_connection().await?;
            let field_count: u8 = conn
                .hset(collection, id_cpy.as_str(), true_value.encode_to_vec())
                .await?;
            
            Ok(true_value)

        })

    }

    fn delete(id: &str) -> Pin<Box<dyn Future<Output = Result<T, Error>>>> {
        
        let id_cpy = id.to_string();
        Box::pin(async move {
            let collection = Self::get_collection_name().await?;

            let res = match Self::read(id_cpy.as_str()).await? {
                Some(res)=>Ok(res),
                None=>Err(
                    anyhow!("No object with id {} found in collection {}", 
                    id_cpy.as_str(), 
                    collection.as_str()
                ))
            }?;

            let mut conn = Self::get_connection().await?;
            let field_count: u8 = conn
                .hdel(collection, id_cpy.as_str())
                .await?;
            
            Ok(res)
              
        })

    }
}