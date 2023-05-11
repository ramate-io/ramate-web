use async_trait::async_trait;

#[async_trait]
pub trait Acls : Send + Sync {

    async fn add<'a, T>(&self, key : &str, ids : T) -> 
    Result<&Self, Box<dyn std::error::Error>>
    where T : IntoIterator<Item=&'a str> + Send + Sync ;

    async fn contains(&self, key : &str, id : &str) -> Result<bool, Box<dyn std::error::Error>>;

    // right now we're computing this in the server program
    // an alternative would be take the ids here as sets and intersect them in the redis space
    // however, this doesn't end up being all that useful because one way or another we wil have to compute the 
    // the complete iterator of ids using recursion in program space
    async fn intersects<'a, T>(&self, key : &str, ids : T)->
    Result<bool, Box<dyn std::error::Error>>
    where T : IntoIterator<Item=&'a str> + Send + Sync;

    async fn empty(&self, key : &str) -> Result<&Self,  Box<dyn std::error::Error>>;

}