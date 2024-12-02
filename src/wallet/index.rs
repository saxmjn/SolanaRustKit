use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub struct Wallet {
    pub keypair: Keypair,
    pub public_key: Pubkey,
}

impl Wallet {
    /// Load a keypair from a JSON file.
    pub fn load_from_file(file_path: &str) -> Wallet {
        let keypair = Keypair::from_bytes(
            &bs58::decode(fs::read_to_string(file_path).expect("Unable to read keypair file"))
                .into_vec()
                .expect("Failed to decode keypair"),
        )
        .expect("Failed to parse keypair");

        Wallet {
            public_key: keypair.pubkey(),
            keypair,
        }
    }

    /// Create a new keypair.
    pub fn load_new() -> Wallet {
        let keypair = Keypair::new();
        Wallet {
            public_key: keypair.pubkey(),
            keypair,
        }
    }

    /// Create a keypair from a private key (hex).
    pub fn load_from_hex(private_key_hex: &str) -> Wallet {
        let private_key_bytes = hex::decode(private_key_hex).expect("Invalid hex string");
        let keypair = Keypair::from_bytes(&private_key_bytes).expect("Failed to create keypair");

        println!("Wallet Pubkey: {}", keypair.pubkey());

        Wallet {
            public_key: keypair.pubkey(),
            keypair,
        }
    }

    pub fn load_from_bytes(base58_key: &[u8]) -> Wallet {

        // Decode the Base58 string into a byte vector
        let private_key_bytes = bs58::decode(base58_key)
            .into_vec()
            .expect("Failed to decode Base58 private key");

        // Create a keypair from the byte array
        let keypair = Keypair::from_bytes(&private_key_bytes)
            .expect("Failed to create keypair from decoded bytes");

        println!("Wallet Pubkey: {}", keypair.pubkey());

        Wallet {
            public_key: keypair.pubkey(),
            keypair,
        }
    }
}
