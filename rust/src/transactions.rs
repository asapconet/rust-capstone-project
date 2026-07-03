use bitcoincore_rpc::bitcoin::{Address, Amount, Txid};
use bitcoincore_rpc::{Client, Result, RpcApi};
use serde::Deserialize;
use serde_json::json;

// You can use calls not provided in RPC lib API using the generic `call` function.
// An example of using the `send` RPC call, which doesn't have exposed API.
// You can also use serde_json `Deserialize` derivation to capture the returned json result.
pub fn send_btc(rpc: &Client, amount: Amount, addr: &Address) -> Result<Txid> {
    // I have to set this outside so i can debug properly in future
    let recipient = addr.to_string();

    //I have to set this outside so i can debug properly in future
    // it allows me to use the wallet without needing to enter the passphrase every time
    rpc.call::<serde_json::Value>(
        "walletpassphrase",
        &[
            json!("traders-or-miners-wallet-are-sometimes-unique"), // this is same as that used in the wallet creation
            json!(10), // seconds until it auto-locks again
        ],
    )?;

    let args = [
        json!({recipient: amount.to_btc()}), // recipient address -- it has be updated to include the amount type and values
        json!(null),                         // conf target
        json!(null),                         // estimate mode
        json!(null),                         // fee rate in sats/vb
        json!(null),                         // Empty option object
    ];

    #[derive(Deserialize)]
    struct SendResult {
        complete: bool,
        txid: Txid,
    }

    let send_result = rpc.call::<SendResult>("send", &args)?;
    assert!(send_result.complete);
    Ok(send_result.txid)
}

// this helper checks the memory pool for the matching transacton id I pass to it
// it return a boolean [true] if the transaction is found in the mempool, [false] otherwise
pub fn check_memory_tx(rpc: &Client, txid: &Txid) -> Result<bool> {
    let mempool_tx = rpc.get_raw_mempool()?;

    if mempool_tx.contains(&txid) {
        println!("transaction: {} found succfully in the mempool", txid);
        Ok(true)
    } else {
        println!("transaction: {} not found in the mempool", txid);
        Ok(false)
    }
}
