#[macro_use]
extern crate serde_derive;
mod blockchain_info;
mod blockchain_status;
mod blockchain_address;
mod blockchain_transaction;

use {
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transaction::BlockchainTransaction,
    dotenv,
};

fn blockchain_info_app(address: &str) {
    match blockchain_info::blockchain_status_request() {
        Ok(blockchain_status) => {
            println!("\n\nQuerying {} - chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);

            match blockchain_info::blockchain_address_request(address) {
                Ok(blockchain_address) => {
                    println!("\n\nAnalyzing transactions for Bitcoin address {}", &blockchain_address.address);

                    let sleep_time = time::Duration::from_millis(2500);
                    thread::sleep(sleep_time);

                    println!("\nYou have a total of {} transactions!", &blockchain_address.txids.len());

                    println!("\n Do you want to query these transactions? (y/n)\n");

                    let mut command = String::new();
                    let _ = io::stdin().read_line(&mut command);

                    if command.trim().eq("y"){

        println!("\nWe will look up the following transactions:\n");
        println!("{:#?}", &blockchain_address.txids);
        thread::sleep(sleep_time);

        let mut balance: i32 = 0;
        for tx_id in &blockchain_address.txids {

            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;

             let blockchain_transaction_result = blockchain_info::blockchain_transaction_request(&tx_id);

            match blockchain_transaction_result {
                Ok(blockchain_transaction) => {
                    // Handle the BlockchainTransaction
                    println!("-----------------------------------------------------");
                    println!("TX ID:           {}", &blockchain_transaction.txid);
                    println!("SATOSHIS IN:     {}", &subtotal_vout);
                    println!("SATOSHIS OUT:    {}", &subtotal_vin);
                    println!("BALANCE:         {}", &balance);
                    println!("-----------------------------------------------------");
                }

        println!("CURRENT BALANCE:     {}", &balance);
        println!("         IN BTC:     {}\n\n", balance as f32 * 0.00000001);
    }
}
Err(err) => {
                    eprintln!("Error retrieving blockchain address: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error retrieving blockchain status: {}", err);
        }
    }
}

use std::env;
fn main() {
    let entered_address = dotenv::var("WALLET").expect("Error reading env var");
    blockchain_info_app(&entered_address);
    env::set_var("RUST_BACKTRACE", "1");
    
}