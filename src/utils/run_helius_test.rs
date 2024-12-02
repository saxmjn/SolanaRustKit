use helius::error::Result;
use helius::types::{Cluster, GetAssetRequest, GetAssetResponseForAsset};
use helius::Helius;

#[tokio::main]
pub async fn run_helius_test() -> Result<()> {
    let api_key: &str = "5feb3b80-ae4a-42a1-8e51-6a5b7f382d99";
    let cluster: Cluster = Cluster::MainnetBeta;

    let helius: Helius = Helius::new(api_key, cluster).unwrap();

    let request: GetAssetRequest = GetAssetRequest {
        id: "F9Lw3ki3hJ7PF9HQXsBzoY8GyE6sPoEZZdXJBsTTD2rk".to_string(),
        display_options: None,
    };

    let response: Result<Option<GetAssetResponseForAsset>> = helius.rpc().get_asset(request).await;

    match response {
        Ok(Some(asset)) => {
            println!("Asset: {:?}", asset);
        },
        Ok(None) => println!("No asset found."),
        Err(e) => println!("Error retrieving asset: {:?}", e),
    }

    Ok(())
}