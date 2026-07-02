use bitcoincore_rpc::bitcoin::{Address, Amount};
use bitcoincore_rpc::{Client, Result, RpcApi};
use serde::Deserialize;
use serde_json::json;

// You can use calls not provided in RPC lib API using the generic `call` function.
// An example of using the `send` RPC call, which doesn't have exposed API.
// You can also use serde_json `Deserialize` derivation to capture the returned json result.
pub fn send_btc(rpc: &Client, amount: Amount, addr: &Address) -> Result<String> {
    // I have to set this outside so i can debug properly in future
    let recipient = addr.to_string();
    let args = [
        json!([{recipient: amount.to_btc()}]), // recipient address -- it has be updated to include the amount type and values
        json!(null),                           // conf target
        json!(null),                           // estimate mode
        json!(null),                           // fee rate in sats/vb
        json!(null),                           // Empty option object
    ];

    #[derive(Deserialize)]
    struct SendResult {
        complete: bool,
        txid: String,
    }
    let send_result = rpc.call::<SendResult>("send", &args)?;
    assert!(send_result.complete);
    Ok(send_result.txid)
}
