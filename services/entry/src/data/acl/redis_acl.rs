use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use std::rc::{Rc};

use crate::util::{
    acl::acl::Acls,
    redis::redis::{REDIS, RedisConnectionGet},
};

use async_trait::async_trait;
use redis::{AsyncCommands, Connection};
use tokio::try_join;
use std::marker::{Send, Sync};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{Receiver};
use anyhow::{Error};

pub struct RedisAcls {}

impl RedisAcls {

}

#[async_trait]
impl Acls for RedisAcls {

    async fn add<'a>(
        &self, 
        key: &str, 
        mut ids: Receiver<&'a str>
    ) -> Result<&Self, Error>{
        let mut connection = REDIS.get_connection().await?;
        let mut pipeline = redis::pipe();

        while let Some(id) = ids.recv().await {
            pipeline.sadd(&key, id);
        }

        let _: Vec<bool> = pipeline.query_async(connection.borrow_mut()).await?;
        Ok(self)
    }

    async fn contains(&self, key: &str, id: &str) -> Result<bool, Error> {
        let mut connection = REDIS.get_connection().await?;
        let contains: bool = redis::cmd("SISMEMBER")
            .arg(&key)
            .arg(&id)
            .query_async(connection.borrow_mut())
            .await?;
        Ok(contains)
    }

    async fn intersects<'a>(
        &self,
        key: &str,
        mut ids: Receiver<&'a str>,
    ) -> Result<bool, Error> {
        let mut connection = REDIS.get_connection().await?;
        let mut pipeline = redis::pipe();
    
        let tkey = format!("temp_set_{}", SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
        let temp_key = Arc::new(tkey.as_str());
    
        // Read IDs from the receiver and add them to the pipeline
        while let Some(id) = ids.recv().await {
            pipeline.sadd(&*temp_key, id).ignore();
        }
    
        pipeline.cmd("SINTERCARD").arg(2).arg(&key).arg(&*temp_key);
        pipeline.del(&*temp_key).ignore();
    
        let (result,): (u8,) = pipeline.query_async(connection.borrow_mut()).await?;
        let intersects = result > 0;
    
        Ok(intersects)
    }    

    async fn empty(&self, key: &str) -> Result<&Self, Error> {
        let mut connection = REDIS.get_connection().await?;
        redis::cmd("DEL").arg(&key).query_async(connection.borrow_mut()).await?;
        Ok(self)
    }
}