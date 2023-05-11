use entry::correspondence_client::CorrespondenceClient;

mod entry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = CorrespondenceClient::connect("http://[::1]:50051").await?;

    Ok(())
}
