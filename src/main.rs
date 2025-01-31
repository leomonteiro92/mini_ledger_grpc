use std::sync::Arc;

use base::storage::InMemoryStorage;
use server::{ledger::mini_ledger_server::MiniLedgerServer, MiniLedgerService};
use tokio::sync::Mutex;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server()
        .await
        .map_err(|err| format!("Failed to start server: {}", err))?;
    Ok(())
}

async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse().unwrap();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(server::ledger::FILE_DESCRIPTOR_SET)
        .build_v1()
        .map_err(|err| format!("Failed to build reflection service: {}", err))?;

    let storage = Arc::new(Mutex::new(InMemoryStorage::new()));
    let ledger_service = MiniLedgerService::new(storage);

    tonic::transport::Server::builder()
        .add_service(MiniLedgerServer::new(ledger_service))
        .add_service(reflection_service)
        .serve(addr)
        .await
        .map_err(|err| format!("Failed to start server: {}", err))?;
    println!("Server started at {}", addr);
    Ok(())
}
