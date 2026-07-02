#![allow(unused)]
use bitcoin::hex::DisplayHex;
use bitcoincore_rpc::bitcoin::{Amount, Network};
use bitcoincore_rpc::RpcApi;
use serde_json::{json, Error};
use std::fs::File;
use std::io::Write;

mod rpc;
mod transactions;
mod wallets;

use crate::rpc::{default_node, wallet_node};
use crate::transactions::send_btc;
use crate::wallets::{create_receiving_addy, generate_balance, load_or_create_wallet};

fn main() -> bitcoincore_rpc::Result<()> {
    let initialize = default_node()?;

    // Create/Load the wallets, named 'Miner' and 'Trader'.
    // Have logic to optionally create/load them if they do not exist or not loaded already.
    let wallets = ("Miner", "Trader");
    load_or_create_wallet(&initialize, wallets.0)?;
    load_or_create_wallet(&initialize, wallets.1)?;

    let miner_rpc = wallet_node(wallets.0)?;
    let trader_rpc = wallet_node(wallets.1)?;

    // Generate spendable balances in the Miner wallet.
    // How many blocks needs to be mined? [answer == 100]
    generate_balance(&miner_rpc, wallets.0)?;

    // Load Trader wallet and generate a new address
    let trader_addy = create_receiving_addy(&trader_rpc, wallets.1)?;
    println!("Trader Address: {}", &trader_addy);

    // Send 20 BTC from Miner to Trader
    let amount_to_send = Amount::from_sat(2_000_000_000);

    // now I will send the transaction and get the transaction ID back so i can use it to check the mempool
    let txid = send_btc(&miner_rpc, amount_to_send, &trader_addy)?;
    println!("Transaction ID: {}", txid);

    // Check transaction in mempool

    // Mine 1 block to confirm the transaction

    // Extract all required transaction details

    // Write the data to ../out.txt in the specified format given in readme.md

    Ok(())
}
