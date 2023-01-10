mod helpers;
mod models;
mod multisig;
mod types;

pub use models::account::{FlairAccount, FlairAddress, FlairPrivate, FlairPublic};
pub use models::mnemonic::SecretPhrase;
use multisig::approve::approve_multisig_params;
pub use multisig::construct::create_multisig_params;
use multisig::propose::propose_multisig_params;
pub use types::{ChainType, WalletType};

pub fn multisig_send_propose_params(to: String, value: String) -> String {
    propose_multisig_params(0, None, to, Some(value))
}

pub fn multisig_approve_params(txnid: i64) -> String {
    approve_multisig_params(txnid)
}
