use bitcoincore_rpc::bitcoin::{Address, Amount};
use bitcoincore_rpc::Result;
use bitcoincore_rpc::{Client, RpcApi};

// this function is meant to load and or create the miner or trader wallet
pub fn load_or_create_wallet(rpc: &Client, wallet_label: &str) -> Result<()> {
    let disable_private_keys = Some(false);
    let blank = Some(false);
    let passphrase = Some("traders-or-miners-wallet-are-sometimes-unique");
    let avoid_reuse = Some(true);

    //these are the initial loaders the check weather the wallet exist or not
    if let Err(_) = rpc.load_wallet(&wallet_label) {
        //this checks load and or create a wallet from whatever name we pass to the function
        // if it exist we continue and if it fails we create a new wallet based on the name stng passed
        rpc.create_wallet(
            &wallet_label,
            disable_private_keys,
            blank,
            passphrase,
            avoid_reuse,
        )?;
    }
    Ok(())
}

// this function generates the spendable balance from the wallet after 100 blocks has been mined
// the 101th block gets mined thereafter and return the balance left to spend
pub fn generate_balance(rpc: &Client, miner_wallet: &str) -> Result<Amount> {
    let addy = rpc
        .get_new_address(Some(miner_wallet), None)?
        .assume_checked(); // I am using that with the assumptiion that we are on the Regtest network and not main or any type

    //this should mine after the 100th block level and give back the hashes to be used to get the bal
    let block_hash = rpc.generate_to_address(101, &addy)?;
    println!(
        "Mined {} blocks, the last block is {:?}",
        block_hash.len(),
        block_hash.last()
    );

    let bal = rpc.get_balance(None, None)?;
    Ok(bal)
}

// this function creates a transaction address assuming the wallet has been loaded and or created
pub fn create_receiving_addy(rpc: &Client, wallet_label: &str) -> Result<Address> {
    let new_addr = rpc
        .get_new_address(Some(wallet_label), None)?
        .assume_checked(); // the assumption is that we are on the Regtest network and not main or any type
    Ok(new_addr)
}
