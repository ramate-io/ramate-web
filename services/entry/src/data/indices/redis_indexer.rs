use redis::AsyncCommands;
use std::pin::Pin;
use crate::util::indices::index::{AsyncIndexer};
use crate::util::redis::redis::{REDIS, RedisConnectionGet};
use anyhow::{Error};
use async_trait::async_trait;
use tokio::sync::mpsc::{channel, Receiver};
use futures_core::Future;
use std::sync::{Arc};

const INDEX_BACK_LINK : &str = "<*ibl>";

#[async_trait]
pub trait RedisAsyncIndexer {

    async fn get_backlink_id(resource_id : &str)->Result<String, Error>{
        Ok(format!("{}.{}", INDEX_BACK_LINK, resource_id))
    }

}

impl<T> AsyncIndexer for T
where
    T: RedisAsyncIndexer,
{
    fn link<'life0,'life1,'async_trait>(index_id: &'life0 str,resource_id: &'life1 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait {
        Box::pin(async move {
            let mut conn = REDIS.get_connection().await?;
            conn.sadd(index_id, resource_id).await?;
            let backlink_id = Self::get_backlink_id(resource_id).await?;
            conn.sadd(backlink_id, index_id).await?;
            Ok(true)
        })
    }

    fn unlink<'life0,'life1,'async_trait>(index_id: &'life0 str,resource_id: &'life1 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait {
        Box::pin(async move {
            let mut conn = REDIS.get_connection().await?;
            conn.srem(index_id, resource_id).await?;
            let backlink_id = Self::get_backlink_id(resource_id).await?;
            conn.srem(backlink_id, index_id).await?;
            Ok(true)
        })
    }

    fn get_index_ids<'life0,'async_trait>(resource_id: &'life0 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Receiver<Result<String,Error> > ,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait {
        Box::pin(async move {
            
            let (sender, receiver) = channel(1);
            let rid = resource_id.to_string();
            tokio::spawn(async move {
                let mut cursor: i64 = 0;
                let backlink_id = Self::get_backlink_id(&rid).await?;
                loop {
                    let (new_cursor, index_ids): (i64, Vec<String>) = {
                        let mut conn = REDIS.get_connection().await?;
                        redis::cmd("SSCAN")
                            .arg(backlink_id.clone())
                            .arg(cursor)
                            .query_async(&mut conn)
                            .await?
                    };
                    for index_id in index_ids {
                        sender.send(Ok(index_id)).await?;
                    }
                    if new_cursor == 0 {
                        break;
                    }
                    cursor = new_cursor;
                }
                Ok::<bool, Error>(true)
            });
            
            Ok(receiver)
        })
    }

    fn get_resource_ids<'life0,'async_trait>(index_id: &'life0 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Receiver<Result<String,Error> > ,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait {
        Box::pin(async move {
            let (sender, receiver) = channel(1);
            let mut cursor: i64 = 0;
            let iid = index_id.to_string();
            tokio::spawn(async move {
                loop {
                    let (new_cursor, resource_ids): (i64, Vec<String>) = {
                        let mut conn = REDIS.get_connection().await?;
                        redis::cmd("SSCAN")
                            .arg(&iid)
                            .arg(cursor)
                            .query_async(&mut conn)
                            .await?
                    };
                    for resource_id in resource_ids {
                        sender.send(Ok(resource_id)).await?;
                    }
                    if new_cursor == 0 {
                        break;
                    }
                    cursor = new_cursor;
                }
                Ok::<bool, Error>(true)
            });
    
            Ok(receiver)
        })
    }
}