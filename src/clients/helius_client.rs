// src/helius_client.rs
use helius::Helius;
use helius::error::Result;
use helius::types::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    // Global static instance of the Helius client
    pub static ref HELIUS_CLIENT: Arc<Mutex<Helius>> = Arc::new(Mutex::new(create_helius_client()));
}

fn create_helius_client() -> Helius {
    let api_key: &str = "YOUR_API_KEY";
    let cluster: Cluster = Cluster::Devnet;
    Helius::new(api_key, cluster).unwrap()
}