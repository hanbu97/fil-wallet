use crate::{
    models::account::{FlairAccount, FlairAddress, FlairPrivate, FlairPublic},
    types::WalletType,
};
use anyhow::Context;
use bls_signatures::{PrivateKey as BlsPrivate, Serialize};
use fvm_shared::address::Address;
use libsecp256k1::{PublicKey as SecpPublic, SecretKey as SecpPrivate};

/// private key -> public key -> filecoin address -> account

/// Generate a new private key
pub fn generate_private(wallet_type: &WalletType) -> anyhow::Result<FlairPrivate> {
    let rng = &mut rand::rngs::OsRng::default();
    match wallet_type {
        WalletType::Bls => {
            let key = BlsPrivate::generate(rng);
            Ok(key.as_bytes().into())
        }
        WalletType::Secp256k1 => {
            let key = SecpPrivate::random(rng);
            Ok(key.serialize().to_vec().into())
        }
    }
}

/// Generate a new public key from supplied private key
pub fn generate_public(
    wallet_type: &WalletType,
    private_key: &FlairPrivate,
) -> anyhow::Result<FlairPublic> {
    match wallet_type {
        WalletType::Bls => {
            let pkey = private_key.to_vec();
            Ok(BlsPrivate::from_bytes(&pkey)?
                .public_key()
                .as_bytes()
                .into())
        }
        WalletType::Secp256k1 => {
            let private_key = SecpPrivate::parse_slice(&private_key.to_vec())?;
            let public_key = SecpPublic::from_secret_key(&private_key);
            Ok(public_key.serialize().to_vec().into())
        }
    }
}

/// Generate a new private key from supplied public key
pub fn generate_address(
    wallet_type: &WalletType,
    public_key: &FlairPublic,
) -> anyhow::Result<FlairAddress> {
    match wallet_type {
        WalletType::Bls => {
            let addr = Address::new_bls(&public_key.to_vec())?;
            Ok(addr.into())
        }
        WalletType::Secp256k1 => {
            let addr = Address::new_secp256k1(&public_key.to_vec())?;
            Ok(addr.into())
        }
    }
}

/// Generate a new account from supplied address
pub fn generate_account_from_address(
    wallet_type: &WalletType,
    address: FlairAddress,
) -> anyhow::Result<FlairAccount> {
    Ok(FlairAccount::from_address(*wallet_type, address))
}

/// Generate a new account from supplied public key
pub fn generate_account_from_public(
    wallet_type: &WalletType,
    public_key: &FlairPublic,
) -> anyhow::Result<FlairAccount> {
    let address = generate_address(wallet_type, public_key)?;
    let mut account = generate_account_from_address(wallet_type, address)?;
    account.set_public(public_key.clone());
    Ok(account)
}

/// Generate a new account from private key
pub fn generate_account_from_private(
    wallet_type: &WalletType,
    private_key: &FlairPrivate,
) -> anyhow::Result<FlairAccount> {
    let public_key = generate_public(wallet_type, private_key)?;
    let address = generate_address(wallet_type, &public_key)?;
    let mut account = generate_account_from_address(wallet_type, address)?;
    account.set_private(private_key.clone());
    account.set_public(public_key);
    Ok(account)
}

pub fn parse_private_key_string(key: &str) -> anyhow::Result<FlairAccount> {
    let decoded_key: serde_json::Value = serde_json::from_str(key)?;

    dbg!(&decoded_key);

    let pkey = decoded_key
        .get("PrivateKey")
        .context("Private key format error!")?
        .as_str()
        .context("Private key format error!")?
        .trim();

    let wallet_type = match decoded_key.get("Type") {
        Some(t) if t.to_string().trim() != "null" => {
            let tt = t.to_string();
            if tt.contains("bls") {
                WalletType::Bls
            } else if tt.contains("secp256k1") {
                WalletType::Secp256k1
            } else {
                return Err(anyhow::anyhow!("Private key format error!"));
            }
        }
        _ => WalletType::Secp256k1,
    };
    dbg!(&wallet_type);

    // let wallet_type = match rtype.as_str() {
    //     "secp256k1" => WalletType::Secp256k1,
    //     "bls" => WalletType::Bls,
    //     _ => return Err(anyhow::anyhow!("Private key format error!")),
    // };

    let mut private_key: FlairPrivate = base64::decode(pkey)?.into();
    private_key.set_type(wallet_type);
    let account = generate_account_from_private(&wallet_type, &private_key)?;

    Ok(account)
}

/// Exported wallet private key from wallets/lotus
pub fn generate_account_from_encoded_string(key: &str) -> anyhow::Result<FlairAccount> {
    let key = key.trim();
    let decoded_key = hex::decode(key)?;
    let key = String::from_utf8(decoded_key)?;
    let account = parse_private_key_string(&key)?;

    Ok(account)
}

/// Generate a new account from nothing
pub fn generate_account(wallet_type: &WalletType) -> anyhow::Result<FlairAccount> {
    let private_key = generate_private(wallet_type)?;
    let public_key = generate_public(wallet_type, &private_key)?;
    let address = generate_address(wallet_type, &public_key)?;
    let mut account = generate_account_from_address(wallet_type, address)?;
    account.set_private(private_key);
    account.set_public(public_key);
    Ok(account)
}
