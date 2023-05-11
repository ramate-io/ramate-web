#[cfg(test)]
mod redis_crud_tests {

    extern crate dotenv;

    use super::super::redis_collection::{RedisAsyncCollection, RedisAsyncCollectionAdmin};
    use crate::entry::Project;
    use crate::util::crud::crud::AsyncCrud;
    use crate::util::redis::redis::{RedisConnection, RedisConnectionGet};
    use crate::util::collection::collection::{AsyncNamedCollection, AsyncCollection, AsyncCollectionAdmin};
    use crate::util::redis::redis::REDIS;
    use uuid::Uuid;
    use dotenv::dotenv;
    use anyhow::Error;
    use tokio;

    impl RedisAsyncCollection for Project {
        
    }

    impl RedisAsyncCollectionAdmin for Project {

    }

    #[tokio::test]
    async fn test_create()->Result<(), Box<dyn std::error::Error>>{

        dotenv().ok();

        let _ = Project::clear().await?;
    
        for i in 0..25 {
            Project::create(Project {
                id : "irr".to_string(),
                description : "first_project".to_string(),
                milestones : vec![]
            }).await?;
        }

        let mut projects = Project::get_all("nocursor").await?;
        let mut i = 0;
        while let Some(project) = projects.recv().await {

            println!("{:?}", project?);
            i = i + 1;

        }

        assert_eq!(i, 25);

        let _ = Project::clear().await?;

        Ok(())

    }

}