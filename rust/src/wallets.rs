use bitcoincore_rpc::Result;
use bitcoincore_rpc::{Client, RpcApi};

// this function is meant to load and or create the miner or trader wallet
pub fn load_or_create_wallet(rpc: &Client, wallet_name: &str) -> Result<()> {
    let disable_private_keys = Some(false);
    let blank = Some(false);
    let passphrase = Some("traders-or-miners-wallet-are-sometimes-unique");
    let avoid_reuse = Some(true);

    //these are the initial loaders the check weather the wallet exist or not
    if let Err(_) = rpc.load_wallet(&wallet_name) {
        //this checks load and or create a wallet from whatever name we pass to the function
        // if it exist we continue and if it fails we create a new wallet based on the name stng passed
        rpc.create_wallet(
            &wallet_name,
            disable_private_keys,
            blank,
            passphrase,
            avoid_reuse,
        )?;
    }
    Ok(())
}
