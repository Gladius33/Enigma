mod server;
mod consensus;
mod db;

#[tokio::main]
async fn main() {
    server::start_server().await;
}
