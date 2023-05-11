use std::{env, sync::{Arc, RwLock}};
use redis::{ aio::Connection, AsyncCommands};
use lazy_static::lazy_static;
use async_trait::async_trait;
use anyhow::{Error};

#[async_trait]
pub trait RedisConnection {

    async fn get_connection() -> Result<
        Connection, 
        Error
    >;

}

#[async_trait]
pub trait RedisConnectionGet {

    async fn get_connection(&self) -> Result<
        Connection, 
        Error
    >;
    
}

pub struct RedisSingleton {
    connection : RwLock<Option<Arc<RwLock<Connection>>>>
}

impl RedisSingleton {
    fn new() -> Self {
        RedisSingleton {
            connection : RwLock::from(None)
        }
    }
}

impl Default for RedisSingleton {
    fn default() -> Self {
        RedisSingleton::new()
    }
}

pub static REDIS_PATH : &str = "REDIS_PATH";

#[async_trait]
impl RedisConnectionGet for RedisSingleton {

    // get ths connection
    async fn get_connection(&self) -> Result<
        Connection, 
        Error
    > {

        let path = env::var(REDIS_PATH).expect(
            format!( "env variable `{}` not present", REDIS_PATH).as_str()
        );

        let client = redis::Client::open(path)?;
        let connection = client.get_async_connection().await?;

        // self.connection.write().unwrap().replace(connection.clone());
        Ok(connection)
        
    }

}



lazy_static!{
    pub static ref REDIS: RedisSingleton = RedisSingleton::new();
}