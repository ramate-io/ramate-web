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

pub trait RedisAcls {}

pub struct RedisAclsManager {}

impl RedisAcls for RedisAclsManager {}


impl <T> Acls for T
    where
        T  : RedisAcls
{

    fn add<'life0,'async_trait>(resource_id: &'life0 str, mut ids:Receiver<String>) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait {
        Box::pin(async move{

            let mut connection = REDIS.get_connection().await?;
            let mut pipeline = redis::pipe();
    
            while let Some(id) = ids.recv().await {
                pipeline.sadd(&resource_id, id);
            }
    
            let _: Vec<bool> = pipeline.query_async(connection.borrow_mut()).await?;
            Ok(true)

        })
    }
    

    fn contains<'life0,'life1,'async_trait>(resource_id: &'life0 str,id: &'life1 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait {
        
        Box::pin(async move {

            let mut connection = REDIS.get_connection().await?;
            let contains: bool = redis::cmd("SISMEMBER")
                .arg(&resource_id)
                .arg(&id)
                .query_async(connection.borrow_mut())
                .await?;
            Ok(contains)

        })

    }

    fn intersects<'life0,'async_trait>(resource_id: &'life0 str, mut ids:Receiver<String> ,) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait {
        Box::pin(async move {

            let mut connection = REDIS.get_connection().await?;
            let mut pipeline = redis::pipe();
        
            let tresource_id = format!("temp_set_{}", SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
            let temp_resource_id = Arc::new(tresource_id.as_str());
        
            // Read IDs from the receiver and add them to the pipeline
            while let Some(id) = ids.recv().await {
                pipeline.sadd(&*temp_resource_id, id).ignore();
            }
        
            pipeline.cmd("SINTERCARD").arg(2).arg(&resource_id).arg(&*temp_resource_id);
            pipeline.del(&*temp_resource_id).ignore();
        
            let (result,): (u8,) = pipeline.query_async(connection.borrow_mut()).await?;
            let intersects = result > 0;
        
            Ok(intersects)

        })
    }

    fn empty<'life0,'async_trait>(resource_id: &'life0 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait {
        Box::pin(async move {

            let mut connection = REDIS.get_connection().await?;
            redis::cmd("DEL").arg(&resource_id).query_async(connection.borrow_mut()).await?;
            Ok(true)

        })
    }

}