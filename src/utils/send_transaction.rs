use crate::clients::helius_client::HELIUS_CLIENT;

use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::{pubkey::Pubkey, instruction::Instruction, system_instruction::transfer, native_token::LAMPORTS_PER_SOL};
use solana_client::rpc_config::RpcSendTransactionConfig;

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{timeout, sleep};

use helius::types::*;

pub async fn send_transaction(to_pubkey: &str, transfer_amount: u64, transfer_tip: u64) {
  let helius = HELIUS_CLIENT.lock().await;
  let helius_region: &str = "NY";

  // Replace with your actual keypair
  let sender_keypair: Keypair = Keypair::new();
  let sender_pubkey: Pubkey = sender_keypair.pubkey();

  let recipient_pubkey: Pubkey = Pubkey::from_str(to_pubkey).unwrap();

  // Create a simple instruction (transfer amount from from_pubkey to to_pubkey)
  let instructions: Vec<Instruction> = vec![transfer(&sender_pubkey, &recipient_pubkey, transfer_amount)];

  let create_config: CreateSmartTransactionConfig = CreateSmartTransactionConfig {
    instructions,
    signers: vec![Arc::new(sender_keypair)],
    lookup_tables: None,
    fee_payer: None,
  };

  let config: SmartTransactionConfig = SmartTransactionConfig {
    create_config,
    send_options: RpcSendTransactionConfig {
        skip_preflight: true,
        preflight_commitment: None,
        encoding: None,
        max_retries: None,
        min_context_slot: None,
    },
    timeout: Timeout::default(),
  };

  // Send the optimized transaction with a lamport tip using the New York region's API URL
  match helius
    .send_smart_transaction_with_tip(config, Some(transfer_tip), Some(helius_region))
    .await
  {
    Ok(bundle_id) => {
        println!("Transaction sent successfully: {}", bundle_id);
        sleep(Duration::from_secs(5)).await;

        // Get final balances
        let balance_from = helius.connection().get_balance(&sender_pubkey).unwrap_or(0);
        println!(
            "From Wallet Balance: {} SOL",
            balance_from as f64 / LAMPORTS_PER_SOL as f64
        );

        let balance_to = helius.connection().get_balance(&recipient_pubkey).unwrap_or(0);
        println!("To Wallet Balance: {} SOL", balance_to as f64 / LAMPORTS_PER_SOL as f64);
    }
    Err(e) => {
        eprintln!("Failed to send transaction: {:?}", e);
    }
  }
}