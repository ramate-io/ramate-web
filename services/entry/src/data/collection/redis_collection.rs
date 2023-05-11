use crate::util::collection::collection::{AsyncNamedCollection, AsyncCollection, AsyncCollectionAdmin};
use futures_util::TryStreamExt;
use prost::Message;
use redis::AsyncCommands;
use std::any::type_name;
use crate::util::redis::redis::RedisConnection;
use redis::AsyncIter;
use std::collections::VecDeque;
use tokio::sync::mpsc;
use tokio::task;
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;
use tokio::sync::mpsc::{channel, Receiver};
use anyhow::{anyhow, Error};
use std::sync::{Arc, Mutex};

pub trait ProstAsyncNamedCollection {
    
}

impl <T> AsyncNamedCollection for T 
    where 
        T : ProstAsyncNamedCollection + Message 
{

    fn get_collection_name() -> std::pin::Pin<Box<dyn futures_core::Future<Output = Result<String, Error>>>> {
        
        Box::pin(async move {

            Ok(type_name::<Self>().to_string())

        })

    }

}

pub trait RedisAsyncCollection {
    
}

impl<T> AsyncCollection<T> for T
where
    T: RedisAsyncCollection + Message + AsyncNamedCollection + RedisConnection + Send + Default + 'static,
{
    
    fn get_all(cursor_id: &str)
        -> Pin<Box<dyn 
            futures_core::Future<Output = Result<
                Receiver<Result<T, Error>>,
                Error
            >>
        >> {
        
            Box::pin(async move {
                let collection = Self::get_collection_name().await?;
                let (sender, receiver) = channel::<Result<T, Error>>(10);
    
                tokio::spawn(async move {
                    let mut conn = Self::get_connection().await?;
                    let mut scan: AsyncIter<(Vec<u8>, Vec<u8>)> = conn.hscan(collection.clone()).await?;
                    while let Some((_, buf)) = scan.next().await {

                        let result = match Self::decode(VecDeque::from(buf)) {
                            Ok(res) => Ok(res),
                            Err(_) => Err(anyhow!(
                                "Could not decode message from collection: {}",
                                collection.as_str(),
                            )),
                        };
    
                        if sender.send(result).await.is_err() {
                            break;
                        }
                    }

                    Ok::<(), Error>(())

                });
    
                Ok(receiver)

            })

        }


       
}

pub trait RedisAsyncCollectionAdmin {}

impl<T> AsyncCollectionAdmin for T
where
    T: RedisAsyncCollection + Message + AsyncNamedCollection + RedisConnection + Send + Default + 'static,
{

    fn clear()
       -> Pin<Box<dyn 
           futures_core::Future<Output = Result<
               bool,
               Error
           >>
       >> {

        Box::pin(async move {

            let collection = Self::get_collection_name().await?;
            let mut conn = Self::get_connection().await?;
            let _ : u8 = conn.del(collection).await?;

            Ok(true)

        })

    }

}
