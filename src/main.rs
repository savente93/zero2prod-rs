use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to buind random port");

    let server = zero2prod::run(listener).expect("Faild to bind server address");

    server.await
}
