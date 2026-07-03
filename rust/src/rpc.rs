use bitcoincore_rpc::{Auth, Client, Result, RpcApi};

// Node access params
const RPC_URL: &str = "http://127.0.0.1:18443"; // Default regtest RPC port
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";

// this is the original connection established between the Rust client and the Bitcoin Core RPC server
// it is used as the default node for all RPC calls
pub fn default_node() -> Result<Client> {
    // Connect to Bitcoin Core RPC
    let rpc = Client::new(
        RPC_URL,
        Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
    )?;

    // Get blockchain info
    let blockchain_info = rpc.get_blockchain_info()?;
    println!("Blockchain Info: {:?}", blockchain_info);

    Ok(rpc)
}

// this is a helper function to connect to a wallet node by name
// so I don't have to remember the wallet URL format. it builds the URL dynamically
pub fn wallet_node(wallet_name: &str) -> Result<Client> {
    let wallet_url = format!("{}/wallet/{}", RPC_URL, wallet_name);
    let wallet = Client::new(
        &wallet_url,
        Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
    )?;

    Ok(wallet)
}
