use helius::error::Result;
use helius::types::*;
use helius::Helius;

use solana_client::client_error::ClientError;
use solana_sdk::hash::Hash;

#[tokio::main]
pub async fn get_block_hash() -> Result<()> {
    let api_key: &str = "5feb3b80-ae4a-42a1-8e51-6a5b7f382d99";
    let cluster: Cluster = Cluster::MainnetBeta;

    let helius: Helius = Helius::new(api_key, cluster).unwrap();

    let result: std::result::Result<Hash, ClientError> = helius.connection().get_latest_blockhash();
    println!("{:?}", result);

    Ok(())
}