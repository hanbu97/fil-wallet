use serde::{Deserialize, Serialize};
use std::str::FromStr;
use structopt::StructOpt;

// any error type implementing Display is acceptable.
type ParseError = &'static str;

/// chain network type: mainnet, calibnet
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ChainType {
    #[default]
    Mainnet,
    Calibnet,
}
impl FromStr for ChainType {
    type Err = ParseError;
    fn from_str(chain: &str) -> Result<Self, Self::Err> {
        match chain.to_lowercase().as_str() {
            "mainnet" => Ok(ChainType::Mainnet),
            "calibnet" => Ok(ChainType::Calibnet),
            _ => Err("Chain type not supported"),
        }
    }
}

//StructOpt
#[derive(Debug, Default, StructOpt, PartialEq, Eq, Clone, Copy)]
pub enum WalletType {
    #[default]
    Secp256k1,
    Bls,
}

impl ToString for WalletType {
    fn to_string(&self) -> String {
        match self {
            WalletType::Bls => "bls".to_string(),
            WalletType::Secp256k1 => "secp256k1".to_string(),
        }
    }
}
