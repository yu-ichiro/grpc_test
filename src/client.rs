use std::io::{stdin, stdout, Write};
use crate::grpc::hello::HelloRequest;
use crate::grpc::hello::greeter_client::{GreeterClient};

mod grpc;

trait Strip {
    fn strip_newline(&mut self) -> ();
}

impl Strip for String {
    fn strip_newline(&mut self) -> () {
        if self.ends_with("\n") {
            self.pop();
            if self.ends_with("\r") {
                self.pop();
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let mut line = String::new();

    loop {
        line.clear();
        print!("> ");
        if stdout().flush().is_err() { continue; }
        if stdin().read_line(&mut line).is_err() { continue; }
        line.strip_newline();

        if line.eq("q") { break; }

        let request = tonic::Request::new(HelloRequest {
            name: line.clone().into()
        });

        let response = client.say_hello(request).await?;

        println!("{}", response.into_inner().message);
    }

    Ok(())
}
