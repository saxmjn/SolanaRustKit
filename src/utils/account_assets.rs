use crate::clients::helius_client::HELIUS_CLIENT;

use helius::error::Result;
use helius::types::*;

#[tokio::main]
pub async fn get_account_assets(address: &str) -> Result<()> {
  let helius = HELIUS_CLIENT.lock().await;

  let request: GetAssetsByOwner = GetAssetsByOwner {
    owner_address: address.to_string(),
    page: 1,
    display_options: Some(DisplayOptions {
        show_collection_metadata: true,
        ..Default::default()
    }),
    ..Default::default()
  };

  let response : Result<AssetList> = helius.rpc().get_assets_by_owner(request).await;
  
  println!("Assets: {:?}", response);

   Ok(())
}