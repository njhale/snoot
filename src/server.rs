use tonic::transport::{Server};
use tonic::{Request, Response, Status};

use snoot::snoot_service_server::{SnootService, SnootServiceServer};
use snoot::{Condition, Snoot};

pub mod snoot {
    // Use a tonic macro to populate this module with generated types
    tonic::include_proto!("snoot"); // Must match the proto package name

    pub(crate) const FILE_DESCRIPTOR_SET: &'static [u8] =
        tonic::include_file_descriptor_set!("snoot_descriptor");
}

#[derive(Debug, Default)]
pub struct StaticSnootService{}

#[tonic::async_trait]
impl SnootService for StaticSnootService {
    async fn boop(
        &self,
        request: Request<Snoot>, // Take Request of type Snoot
    ) -> Result<Response<Condition>, Status> { // Return a Response of type Condition
        println!("Boop = {:?}", request);

        let condition = snoot::Condition {
            snoot: request.into_inner().into(),
            booped: 8,
        };

        Ok(Response::new(condition))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(snoot::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let snoot_service = StaticSnootService::default();

    Server::builder()
        .add_service(reflection_service)
        .add_service(SnootServiceServer::new(snoot_service))
        .serve(addr)
        .await?;

    Ok(())
}
