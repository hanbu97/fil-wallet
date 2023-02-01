use crate::bip32::HDPrivKey;
use anyhow::Result;

pub mod bitcoin;

pub trait Wallet: Sized {
    fn from_hd_key(private_key: HDPrivKey) -> Result<Self>;
}
