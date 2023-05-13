use async_trait::async_trait;
use anyhow::{Error};
use tokio::sync::mpsc::{Receiver};
use crate::util::acl::acl::{Acls};
use super::index::{AsyncIndexer, AsyncIndex};

pub trait UserIndex {}

pub struct UserIdx {}

impl UserIndex for UserIdx {}

// impl <T> AsyncIndex for T
// where
    // T : UserIndex + AsyncIndexer + Acls + Send + Sync
// {

    // for now, we will
    // fn manage_link<'life0,'life1,'life2,'async_trait>(&'life0 self,partition_id: &'life1 str,resource_id: &'life2 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<bool,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        

    // }
    
//}
