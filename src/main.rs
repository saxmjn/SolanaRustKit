mod constants;
use constants::*;

// use my_project::utils::account_assets::get_account_assets;
// use my_project::utils::parse_transaction::get_transaction;
// use my_project::utils::account_parse_transactions::get_transactions_history;
// use my_project::utils::block_hash::get_block_hash;
use my_project::wallet::index::Wallet;
// use my_project::utils::send_transaction::send_transaction;

fn main() {
  let _wallet = Wallet::load_from_bytes(constants::_ACC_PRIVATE_KEY_BYTES);


  println!("Hello, world!");
  // let _ = get_account_assets(constants::_SAMPLE_ADDRESS);
  // let _ = get_transactions_history(constants::_SAMPLE_ADDRESS,_BEFORE_TXN);
  // let _ = get_transaction(constants::_SAMPLE_TXN);
  // let _ = get_block_hash();
  // let _ = send_transaction(constants::_TO_ADDRESS, constants::_AMOUNT, constants::_TIP);
}