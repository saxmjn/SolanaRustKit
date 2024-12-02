use helius::error::Result;
use helius::types::*;
use helius::Helius;

#[tokio::main]
pub async fn get_transactions_history(address: &str, before: Option<&str>) -> Result<()> {
    let api_key: &str = "5feb3b80-ae4a-42a1-8e51-6a5b7f382d99";
    let cluster: Cluster = Cluster::MainnetBeta;

    let helius: Helius = Helius::new(api_key, cluster).unwrap();

    let request: ParsedTransactionHistoryRequest = ParsedTransactionHistoryRequest {
        address: address.to_string(),
        before: before.map(|b| b.to_string()),
    };

    let response: Result<Vec<EnhancedTransaction>> = helius.parsed_transaction_history(request).await;

    println!("Txns: {:?}", response);

    Ok(())
}