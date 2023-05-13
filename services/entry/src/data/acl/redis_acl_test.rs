#[cfg(test)]
mod twilio_tests {

    use dotenv::dotenv;
    use crate::util::acl::acl::Acls;

    use super::super::redis_acl::{RedisAcls, RedisAclsManager};
    use tokio::sync::mpsc::{channel};

    #[tokio::test]
    async fn adds_and_contains_acl() ->  Result<(), Box<dyn std::error::Error>> {

        dotenv().ok();

        let (sender, receiver) = channel(1);

        // Create a vector of items
        let vec_items = vec![
            "liam", 
            "kayla", 
            "john"
        ];

        // Spawn a Tokio task to send items to the receiver
        tokio::spawn(async move {
            for item in vec_items {
                // Send each item to the receiver
                sender.send(item.to_string()).await.unwrap();
            }
            // Drop the sender to signal the end of sending
            drop(sender);
        });

        RedisAclsManager::add("hello", receiver).await?;

        assert!(RedisAclsManager::contains("hello", "liam").await?);

        RedisAclsManager::empty("hello").await?;

        assert!(!RedisAclsManager::contains("hello", "liam").await?);

        Ok(())
    }

    #[tokio::test]
    async fn adds_and_intersects_acl() ->  Result<(), Box<dyn std::error::Error>> {

        dotenv().ok();

        let (add_sender, add_receiver) = channel(1);

        // Create a vector of items
        let adds =  vec![
            "liam", 
            "kayla", 
            "john",
            "ramate"
        ];

        // Spawn a Tokio task to send items to the receiver
        tokio::spawn(async move {
            for item in adds {
                // Send each item to the receiver
                add_sender.send(item.to_string()).await.unwrap();
            }
            // Drop the sender to signal the end of sending
            drop(add_sender);
        });

        RedisAclsManager::add("hello", add_receiver).await?;

        // available
        let (avail_sender, avail_receiver) = channel(1);

        // Create a vector of items
        let avail =  vec![
            "john",
            "kyle",
            "ramate"
        ];

        // Spawn a Tokio task to send items to the receiver
        tokio::spawn(async move {
            for item in avail {
                // Send each item to the receiver
                avail_sender.send(item.to_string()).await.unwrap();
            }
            // Drop the sender to signal the end of sending
            drop(avail_sender);
        });

        assert!(RedisAclsManager::intersects("hello", avail_receiver).await?);

        RedisAclsManager::empty("hello").await?;

        // not available
        
        let (navail_sender, navail_receiver) = channel(1);

        // Create a vector of items
        let navail =  vec![
            "john",
            "kyle",
            "ramate"
        ];

        // Spawn a Tokio task to send items to the receiver
        tokio::spawn(async move {
            for item in navail {
                // Send each item to the receiver
                navail_sender.send(item.to_string()).await.unwrap();
            }
            // Drop the sender to signal the end of sending
            drop(navail_sender);
        });

        assert!(!RedisAclsManager::intersects("hello",  navail_receiver).await?);

        Ok(())
    }

}