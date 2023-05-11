use async_trait::async_trait;

#[async_trait]
pub trait AsyncPrivilege {

    async fn add(user_id : &str, privilege_id : &str, resource_id : &str)
    ->Result<bool, Box<dyn std::error::Error>>;
    
    async fn check(user_id : &str, privilege_id : &str, resource_id : &str)
    ->Result<bool, Box<dyn std::error::Error>>;

}