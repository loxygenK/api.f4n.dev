mod server;
mod model;

use server::execute_server;

use crate::server::Host;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    execute_server(Host::Localhost, 8000)
        .await
        .expect("Server failed");
}
