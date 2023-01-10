mod helpers;
mod message;
mod models;
mod multisig;
mod types;

use message::get_message_cid;
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

pub fn message_cid(
    from: String,
    to: String,
    nonce: u64,
    value: String,
    method: u64,
    params: String,
    gas_limit: i64,
    gas_fee_cap: String,
    gas_premium: String,
) -> String {
    get_message_cid(
        from,
        to,
        nonce,
        value,
        method,
        params,
        gas_limit,
        gas_fee_cap,
        gas_premium,
    )
}
