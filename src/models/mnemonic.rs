use bip39::{Language, Mnemonic, MnemonicType};

use crate::{
    helpers::mnemonic::{gen_account, gen_account_bls},
    FlairAccount, WalletType,
};

#[derive(Debug)]
pub struct SecretPhrase {
    mnemonic: Mnemonic,
    phrase: Vec<String>,
}

impl SecretPhrase {
    /// generate new secret phrase in given language and length
    pub fn generate(language: Language, length: MnemonicType) -> Self {
        let mnemonic = Mnemonic::new(length, language);
        let phrase = mnemonic
            .phrase()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Self { mnemonic, phrase }
    }

    /// generate secret from given phrase(in Englist)
    pub fn generate_from_phrase(phrase: &str) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)?;

        let phrase = mnemonic
            .phrase()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(Self { mnemonic, phrase })
    }

    pub fn phrase_vec(&self) -> Vec<String> {
        self.phrase.clone()
    }

    pub fn phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    pub fn derive_account(
        &self,
        wallet_type: WalletType,
        password: Option<&str>,
    ) -> anyhow::Result<FlairAccount> {
        let password = password.unwrap_or("");

        let account = match wallet_type {
            WalletType::Bls => gen_account_bls(&self.mnemonic, password)?,
            WalletType::Secp256k1 => gen_account(self.phrase(), password)?,
        };

        Ok(account)
    }
}

#[test]
fn test_generate_account() {
    let secret_phrase = SecretPhrase::generate(Language::English, MnemonicType::Words12);
    println!("secret_phrase: {}", secret_phrase.phrase());

    let account_secp = secret_phrase
        .derive_account(WalletType::Secp256k1, None)
        .unwrap();
    println!("Secp256k1 account: {}", account_secp.display());

    let account_bls = secret_phrase.derive_account(WalletType::Bls, None).unwrap();
    println!("Bls account: {}", account_bls.display());

    // secret_phrase: betray ribbon visit topple release angle inspire soul private bottom face buddy
    // Secp256k1 account: f1ihd67zlxq6zbvbtnox4xsjn6htps34ys3mfjv6a
    // Bls account: f3uvn2j4mgp2tz3oiinh3jwnepy6zhhsd76ngec6dgwf5e2wdnfl3ylu7bgqa6jkbqv3e4mua6ectzbleflmtq
}

#[test]
fn test_generate_account_from_phrase() {
    let secret_phrase = SecretPhrase::generate_from_phrase(
        "betray ribbon visit topple release angle inspire soul private bottom face buddy",
    )
    .unwrap();

    let account_secp = secret_phrase
        .derive_account(WalletType::Secp256k1, None)
        .unwrap();
    println!("Secp256k1 account: {}", account_secp.display());

    let account_bls = secret_phrase.derive_account(WalletType::Bls, None).unwrap();
    println!("Bls account: {}", account_bls.display());

    // Secp256k1 account: f1ihd67zlxq6zbvbtnox4xsjn6htps34ys3mfjv6a
    // Bls account: f3uvn2j4mgp2tz3oiinh3jwnepy6zhhsd76ngec6dgwf5e2wdnfl3ylu7bgqa6jkbqv3e4mua6ectzbleflmtq
}
