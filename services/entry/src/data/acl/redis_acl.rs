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

pub struct RedisAcls {}

impl RedisAcls {

}

#[async_trait]
impl Acls for RedisAcls {



    async fn add<'a, T>(&self, key : &str, ids : T) -> 
    Result<&Self, Box<dyn std::error::Error>>
    where T : IntoIterator<Item=&'a str> + Send + Sync {
        let mut connection = REDIS.get_connection().await?;
        let mut pipeline = redis::pipe();
        for id in ids.into_iter() {
            pipeline.sadd(&key, &id);
        }
        let _: Vec<bool> = pipeline.query_async(connection.borrow_mut()).await?;
        Ok(self)
    }

    async fn contains(&self, key: &str, id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut connection = REDIS.get_connection().await?;
        let contains: bool = redis::cmd("SISMEMBER")
            .arg(&key)
            .arg(&id)
            .query_async(connection.borrow_mut())
            .await?;
        Ok(contains)
    }

    async fn intersects<'a, T>(&self, key : &str, ids : T)->
    Result<bool, Box<dyn std::error::Error>>
    where T : IntoIterator<Item=&'a str> + Send + Sync {

        let mut connection = REDIS.get_connection().await?;
        let mut pipeline = redis::pipe();

        let tkey = format!("temp_set_{}",  SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
        let temp_key = Arc::new(tkey.as_str());

        // TODO: this should actually probably be batched
        // we want the performance gains of pipelining
        // however we may want to anticipate large 
        // id trees, in which case pipelining actually 
        // has an undesirably large memory footprint
        for id in ids {
            pipeline.sadd(&*temp_key, id).ignore();
        }
        pipeline.cmd("SINTERCARD").arg(2).arg(&key).arg(&*temp_key);
        pipeline.del(&*temp_key).ignore();

        let (result,): (u8,)  = pipeline.query_async(connection.borrow_mut()).await?;
        let intersects = result > 0;

        Ok(intersects)

    }

    async fn empty(&self, key: &str) -> Result<&Self, Box<dyn std::error::Error>> {
        let mut connection = REDIS.get_connection().await?;
        redis::cmd("DEL").arg(&key).query_async(connection.borrow_mut()).await?;
        Ok(self)
    }
}