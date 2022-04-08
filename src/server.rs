use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::grpc::hello::{HelloReply, HelloRequest};
use crate::grpc::hello::greeter_server::{Greeter, GreeterServer};

mod grpc;

#[derive(Debug, Default)]
pub struct GreeterApi {}

#[tonic::async_trait]
impl Greeter for GreeterApi {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        let hello = request.into_inner();
        println!("Got a request: {:?}", &hello.name);

        let response = HelloReply {
            message: format!("Hello {}!", hello.name).into()
        };

        Ok(Response::new(response))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = GreeterApi::default();

    println!("greeter server listening at {}...", &addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;


    Ok(())
}
