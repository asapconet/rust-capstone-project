use std::fs::File;
use std::io::{Result, Write};

pub struct TxOutputData {
    pub txid: String,
    pub miner_input_address: String,
    pub miner_input_amount: f64,
    pub trader_output_address: String,
    pub trader_output_amount: f64,
    pub miner_change_address: String,
    pub miner_change_amount: f64,
    pub fee: f64,
    pub block_height: u64,
    pub block_hash: String,
}

pub fn write_out_txt(data: &TxOutputData) -> Result<()> {
    let mut file = File::create("../outputs.txt")?;

    writeln!(file, "TxID: {}", data.txid)?;
    writeln!(file, "Miner Input Address: {}", data.miner_input_address)?;
    writeln!(file, "Miner Input Amount: {}", data.miner_input_amount)?;
    writeln!(
        file,
        "Trader Output Address: {}",
        data.trader_output_address
    )?;
    writeln!(file, "Trader Output Amount: {}", data.trader_output_amount)?;
    writeln!(file, "Miner Change Address: {}", data.miner_change_address)?;
    writeln!(file, "Miner Change Amount: {}", data.miner_change_amount)?;
    writeln!(file, "Fee: {}", data.fee)?;
    writeln!(file, "Block Height: {}", data.block_height)?;
    writeln!(file, "Block Hash: {}", data.block_hash)?;
    Ok(())
}
