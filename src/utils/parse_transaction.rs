use crate::clients::helius_client::HELIUS_CLIENT;

use helius::error::Result;
use helius::types::*;

#[tokio::main]
pub async fn get_transaction(transaction: &str) -> Result<()> {
    let helius = HELIUS_CLIENT.lock().await;

    let request: ParseTransactionsRequest = ParseTransactionsRequest {
        transactions: vec![transaction.to_string(),
        ],
    };

    let response: Result<Vec<EnhancedTransaction>> = helius.parse_transactions(request).await;
    println!("Assets: {:?}", response);

    Ok(())
}