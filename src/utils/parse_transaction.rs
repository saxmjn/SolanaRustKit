use helius::error::Result;
use helius::types::*;
use helius::Helius;

#[tokio::main]
pub async fn get_transaction(transaction: &str) -> Result<()> {
    let api_key: &str = "5feb3b80-ae4a-42a1-8e51-6a5b7f382d99";
    let cluster: Cluster = Cluster::MainnetBeta;

    let helius: Helius = Helius::new(api_key, cluster).unwrap();

    let request: ParseTransactionsRequest = ParseTransactionsRequest {
        transactions: vec![transaction.to_string(),
        ],
    };

    let response: Result<Vec<EnhancedTransaction>> = helius.parse_transactions(request).await;
    println!("Assets: {:?}", response);

    Ok(())
}