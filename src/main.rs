mod utils;
mod clients;

use utils::account_assets::get_account_assets;
// use utils::parse_transaction::get_transaction;
// use utils::account_parse_transactions::get_transactions_history;
// use utils::block_hash::get_block_hash;

fn main() {
  let _transaction = "4BCeGnKT6sbxyUJcDkwQQYksEXow5DfkZonRK5t9mNjCue5zCWKPZVfVKJguyDsysgaHz1miue7zk3PUudm58DVh";
  let _address = "2k5AXX4guW9XwRQ1AKCpAuUqgWDpQpwFfpVFh3hnm2Ha";
  let _before = Some("mdo5teJu7pqnjTpzufDKDpaFoy6MbtbP1fdFDKCyVXbPq1V5M6xmmmrinVLnV2bbkHw6UGWv84iu8m4xgrqUTQL");

  
    println!("Hello, world!");
    let _ = get_account_assets(_address);
    // let _ = get_transactions_history(_address,_before);
    // let _ = get_transaction(_transaction);
    // let _ = get_block_hash();
}