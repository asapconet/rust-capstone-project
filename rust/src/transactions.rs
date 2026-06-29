pub struct Transaction {
    fee: u64,
    outputs: Vec<u64>,
    address: String,
}

pub fn extract_fee(transaction: &Transaction) -> u64 {
    transaction.fee
}

pub fn extract_outputs(transaction: &Transaction) -> Vec<u64> {
    transaction.outputs.clone()
}

pub fn extract_address(transaction: &Transaction) -> &str {
    &transaction.address
}
