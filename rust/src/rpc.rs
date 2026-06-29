pub fn create_wallet() -> Wallet {}

pub fn load_wallet() -> Wallet {
    Wallet { balance: 0 }
}
pub fn mine_block() -> Block {
    Block {
        transactions: Vec::new(),
    }
}
pub fn transaction() -> Transaction {
    Transaction {
        amount: 0,
        sender: String::new(),
        recipient: String::new(),
    }
}
