#[cfg(test)]
mod redis_crud_tests {

    extern crate dotenv;

    use super::super::redis_crud::{RedisAsyncCrud};
    use crate::entry::Project;
    use crate::util::crud::{crud::{AsyncCrud}, id::Id};
    use crate::util::redis::redis::{RedisConnection, RedisConnectionGet};
    use crate::util::collection::collection::AsyncNamedCollection;
    use crate::util::redis::redis::REDIS;
    use uuid::Uuid;
    use dotenv::dotenv;
    use anyhow::Error;

    impl RedisConnection for Project {

        fn get_connection<'async_trait>() ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<redis::aio::Connection, Error> > + core::marker::Send+'async_trait> > {
            
            Box::pin(async move {

                REDIS.get_connection().await
    
            })

        }

    }

    impl Id for Project {

        fn generate_id<'async_trait>() ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<String,Error> > + core::marker::Send+'async_trait> > {
            
            Box::pin(async move {

                let id = Uuid::new_v4();

                Ok(id.to_string())
    
            })

        }

        fn get_id<'life0,'async_trait>(&'life0 self) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<String,Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
            
            Box::pin(async move {

                Ok(self.id.to_string())
    
            })

        }

        fn with_id<'life0,'life1,'async_trait>(&'life0 self,id: &'life1 str) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Self, Error> > + core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
            
            Box::pin(async move {

                let mut clone = self.clone();
                clone.id = id.to_string();
                Ok(clone)
    
            })

        }
        
    }

    impl AsyncNamedCollection for Project {
        
        fn get_collection_name() -> std::pin::Pin<Box<dyn futures_core::Future<Output = Result<String, Error>>>> {
            
            Box::pin(async move {
                Ok("Projects".to_string())
            })

        }

    }

    impl RedisAsyncCrud for Project {
        
    }

    #[tokio::test]
    async fn test_create()->Result<(), Box<dyn std::error::Error>>{

        dotenv().ok();

        let project = Project::create(Project { 
            id: "hello".to_string(), 
            description: "first".to_string(), 
            milestones: vec![]
        }).await?;

        let retrieved_project = Project::read(project.id.as_str()).await?;

        match retrieved_project {
            Some(proj)=>{

                assert_eq!(proj, project)
            },
            None=>panic!("no entry")
        };

        Ok(())

    }

    #[tokio::test]
    async fn test_update()->Result<(), Box<dyn std::error::Error>>{

        dotenv().ok();

        let project = Project::create(Project { 
            id: "hello".to_string(), 
            description: "first".to_string(), 
            milestones: vec![]
        }).await?;

        let retrieved_project = Project::read(project.id.as_str()).await?;

        match retrieved_project {
            Some(proj)=>{

                assert_eq!(proj, project)
            },
            None=>panic!("no entry")
        };

        let updated_project = Project {
            id : "any".to_string(),
            description : "updated".to_string(),
            milestones : vec![]
        };

        let sent_updated_project = Project::update(project.id.as_str(), updated_project).await?;

        assert_eq!(sent_updated_project.id, project.id);

        let updated_retrieved_project = Project::read(sent_updated_project.id.as_str()).await?;

        match updated_retrieved_project {
            Some(proj)=>{

                assert_eq!(proj.id, project.id);
                assert_ne!(proj.description, project.description);

            },
            None=>panic!("no entry")
        };

        Ok(())

    }
    
    #[tokio::test]
    async fn test_delete()->Result<(), Box<dyn std::error::Error>>{

        dotenv().ok();

        let project = Project::create(Project { 
            id: "hello".to_string(), 
            description: "first".to_string(), 
            milestones: vec![]
        }).await?;

        let retrieved_project = Project::read(project.id.as_str()).await?;

        match retrieved_project {
            Some(proj)=>{

                assert_eq!(proj, project)
            },
            None=>panic!("no entry")
        };

        let deleted_project = Project::delete(project.id.as_str()).await?;
        assert_eq!(deleted_project.id, project.id);

        let retrieved_project = Project::read(project.id.as_str()).await?;
       
        match retrieved_project {
            Some(proj)=>panic!("Did not delete."),
            None=>{

            }
        };

        Ok(())

    }

}