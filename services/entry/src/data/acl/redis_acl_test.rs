#[cfg(test)]
mod twilio_tests {

    use dotenv::dotenv;
    use crate::util::acl::acl::Acls;

    use super::super::redis_acl::{RedisAcls};

    #[tokio::test]
    async fn adds_and_contains_acl() ->  Result<(), Box<dyn std::error::Error>> {

        dotenv().ok();

        let acls = RedisAcls {};

        acls.add("hello", vec![
            "liam", 
            "kayla", 
            "john"
        ]).await?;

        assert!(acls.contains("hello", "liam").await?);

        acls.empty("hello").await?;

        assert!(!acls.contains("hello", "liam").await?);

        Ok(())
    }

    #[tokio::test]
    async fn adds_and_intersects_acl() ->  Result<(), Box<dyn std::error::Error>> {

        dotenv().ok();

        let acls = RedisAcls {};

        acls.add("hello", vec![
            "liam", 
            "kayla", 
            "john",
            "ramate"
        ]).await?;

        assert!(acls.intersects("hello", vec![
            "john",
            "kyle",
            "ramate"
        ]).await?);

        acls.empty("hello").await?;

        assert!(!acls.intersects("hello",  vec![
            "john",
            "kyle",
            "ramate"
        ]).await?);

        Ok(())
    }

}