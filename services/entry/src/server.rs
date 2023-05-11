use tonic::{transport::Server, Request, Response, Status};

use entry::correspondence_server::{Correspondence, CorrespondenceServer};
use entry::{Contact, ContactMedium};

use std::env;

mod data;
mod util;
mod logic;
mod entry;
mod entry_test;
mod echo_descriptor {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("entry_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(echo_descriptor::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let key : &str = "PORT";
    let port : String = match env::var(key) {
        Ok(val) => val,
        Err(_) => String::from("8080")
    };

    let addr = format!("[::0]:{}", port).parse().unwrap();
    // let echoer = MyEchoer::default();

    println!("EchoerServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(reflection_service)
        //.add_service(tonic_web::enable(
        //    EchoerServer::new(echoer)
        // ))
        .serve(addr)
        .await?;

    Ok(())
}