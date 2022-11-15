use std::collections::VecDeque;

use super::account::FlairAddress;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct FlairWallet {
    accounts: VecDeque<FlairAddress>,
}

impl Default for FlairWallet {
    fn default() -> Self {
        Self::new()
    }
}

impl FlairWallet {
    /// Create an empty wallet
    pub fn new() -> Self {
        Self {
            accounts: VecDeque::new(),
        }
    }

    /// List all addresses stored in the wallet
    pub fn _list(&self) -> anyhow::Result<Vec<FlairAddress>> {
        Ok(self.accounts.to_owned().into())
    }
}
