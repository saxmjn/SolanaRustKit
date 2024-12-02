use crate::clients::helius_client::HELIUS_CLIENT;

use helius::error::Result;
use helius::types::*;

#[tokio::main]
pub async fn get_transactions_history(address: &str, before: Option<&str>) -> Result<()> {
    let helius = HELIUS_CLIENT.lock().await;

    let request: ParsedTransactionHistoryRequest = ParsedTransactionHistoryRequest {
        address: address.to_string(),
        before: before.map(|b| b.to_string()),
    };

    let response: Result<Vec<EnhancedTransaction>> = helius.parsed_transaction_history(request).await;

    println!("Txns: {:?}", response);

    Ok(())
}