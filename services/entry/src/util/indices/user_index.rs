use async_trait::async_trait;
use anyhow::{Error};
use tokio::sync::mpsc::{Receiver, channel};
use crate::util::acl::acl::{Acls};
use crate::util::user::user::{UserExpander};
use super::index::{AsyncIndexer, AsyncIndex};
use std::sync::{Arc};

pub trait UserIndex {}

pub struct UserIdx {}

impl UserIndex for UserIdx {}

impl <T> AsyncIndex for T
where
    T : UserIndex + AsyncIndexer + UserExpander + Acls + Send + Sync
{

    // for now, we will assume the partition id is the user id
    fn manage_link<'life0,'life1,'life2,'async_trait>(
        &'life0 self,
        partition_id: &'life1 str,
        resource_id: &'life2 str
    ) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        
        let pid = partition_id.to_string();
        Box::pin(async move {

            let (sender, mut receiver) = tokio::sync::mpsc::channel(10);

            tokio::spawn(async move {

                let mut users = Self::expand_user(&pid).await?;

                while let Some(user_id_res) = users.recv().await {

                    match  user_id_res {
                        Ok(user_id)=>{
                            let _ = sender.send(user_id.as_str().to_owned()).await?;
                        },
                        Err(msg)=>{

                        }
                    }

                }

                drop(sender);

                Ok::<bool, Error>(true)

            });

            Ok(Self::intersects(resource_id, receiver).await?)

        })

    }

    fn get_resource_ids<'life0,'life1,'async_trait>(&'life0 self,partition_id: &'life1 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Receiver<Result<String,Error> > ,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        
        Self::get_resource_ids(partition_id)

    }
    
}
