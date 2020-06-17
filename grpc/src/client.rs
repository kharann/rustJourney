use hello::say_client::SayClient;
use hello::SayRequest;
use futures::stream::iter;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = SayClient::new(channel);
    // creating a new Request
    //let request = tonic::Request::new(SayRequest {
     //   name: String::from("anshul"),
    //});

    let request = tonic::Request::new(iter(vec![
        SayRequest {
            name: String::from("anshul"),
        },
        SayRequest {
            name: String::from("rahul"),
        },
        SayRequest {
            name: String::from("vijay"),
        },
    ]));

    // sending request and waiting for response
    let response = client.receive_stream(request).await?.into_inner();
    println!("RESPONSE=\n{}", response.message);
    Ok(())
}
