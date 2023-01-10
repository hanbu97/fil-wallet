mod helpers;
mod models;
mod multisig;
mod types;

pub use models::account::{FlairAccount, FlairAddress, FlairPrivate, FlairPublic};
pub use models::mnemonic::SecretPhrase;
pub use multisig::construct::create_multisig_params;
use multisig::propose::multisig_propose_params;
pub use types::{ChainType, WalletType};

pub fn multisig_send_propose_params(to: String, value: String) -> String {
    multisig_propose_params(0, None, to, Some(value))
}
