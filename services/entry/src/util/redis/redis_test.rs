#[cfg(test)]
mod twilio_tests {

    use dotenv::dotenv;
    use crate::util::redis::redis::RedisConnectionGet;

    use redis::AsyncCommands;
    use super::super::redis::{REDIS};

    #[tokio::test]
    async fn sends_message() ->  Result<(), Box<dyn std::error::Error>> {

        dotenv().ok();

        let mut client = REDIS.get_connection().await?;
        let _: () = client.set("a", "a").await?;
        let res : String = client.get("a").await?;
        println!("{:?}", res);

        Ok(())
    }

}