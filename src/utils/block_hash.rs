use crate::clients::helius_client::HELIUS_CLIENT;
use helius::error::Result;

use solana_client::client_error::ClientError;
use solana_sdk::hash::Hash;

#[tokio::main]
pub async fn get_block_hash() -> Result<()> {
    let helius = HELIUS_CLIENT.lock().await;

    let result: std::result::Result<Hash, ClientError> = helius.connection().get_latest_blockhash();
    println!("{:?}", result);

    Ok(())
}