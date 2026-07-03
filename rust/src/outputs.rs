use bitcoincore_rpc::bitcoin::{Address, Txid};
use bitcoincore_rpc::json::GetTransactionResult;
use bitcoincore_rpc::{Client, Result, RpcApi};
use std::fs::File;
use std::io::Write;

pub fn write_tx_output(
    rpc: &Client,
    txid: &Txid,
    trader_addy: &Address,
    tx_details: &GetTransactionResult,
) -> Result<()> {
    let tx = rpc.get_raw_transaction_info(txid, None)?;

    // Previous output being spent (the Miner's input)
    let vin = tx.vin.first().expect("transaction should have one input");

    let prev_tx = rpc.get_raw_transaction_info(&vin.txid.expect("missing previous txid"), None)?;

    let prev_out = &prev_tx.vout[vin.vout.expect("missing previous vout") as usize];

    let miner_input_address = prev_out
        .script_pub_key
        .address
        .clone()
        .expect("missing input address")
        .assume_checked();

    let miner_input_amount = prev_out.value;

    // Find Trader output
    let trader_output = tx
        .vout
        .iter()
        .find(|v| {
            v.script_pub_key
                .address
                .as_ref()
                .map(|a| a.clone().assume_checked() == *trader_addy)
                .unwrap_or(false)
        })
        .expect("missing trader output");

    // this is to find the Miner's change output
    let miner_output = tx
        .vout
        .iter()
        .find(|v| {
            v.script_pub_key
                .address
                .as_ref()
                .map(|a| a.clone().assume_checked() != *trader_addy)
                .unwrap_or(false)
        })
        .expect("missing miner change output");

    let miner_change_address = miner_output
        .script_pub_key
        .address
        .clone()
        .unwrap()
        .assume_checked();

    let mut file = File::create("../out.txt")?;

    writeln!(file, "{txid}")?;
    writeln!(file, "{miner_input_address}")?;
    writeln!(file, "{:.8}", miner_input_amount.to_btc())?;
    writeln!(file, "{trader_addy}")?;
    writeln!(file, "{:.8}", trader_output.value.to_btc())?;
    writeln!(file, "{miner_change_address}")?;
    writeln!(file, "{:.8}", miner_output.value.to_btc())?;
    writeln!(
        file,
        "{:.8}",
        tx_details.fee.expect("missing fee").to_btc().abs()
    )?;
    writeln!(
        file,
        "{}",
        tx_details.info.blockheight.expect("missing block height")
    )?;
    writeln!(
        file,
        "{}",
        tx_details.info.blockhash.expect("missing block hash")
    )?;

    Ok(())
}
