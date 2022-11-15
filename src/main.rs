use bip39::{Language, MnemonicType};
use flair_wallet::{FlairAccount, SecretPhrase, WalletType};

fn main() {
    let in_bls = "7b2254797065223a22626c73222c22507269766174654b6579223a226b434b523969566b73615a6672746b513979356e3269615862317279766d314d37637357456352313142673d227d";
    let account = FlairAccount::import(in_bls).unwrap();
    let out_bls = account.export().unwrap();
    assert_eq!(
        account.display().as_str(),
        "f3vtr3myof7qghg3eq75qfagbfemrhycsqypmz2t6pfwwq5m3w73wlthhbtlrnjpyjxnhwz5pxboonb3bliv6q"
    );
    assert_eq!(out_bls.as_str(), in_bls);

    let in_scep = "7b2254797065223a22736563703235366b31222c22507269766174654b6579223a2235734d384f2b6639554161686d78726d61653533776a667056374338664b6b426c414c4c44366e717a666b3d227d";
    let account = FlairAccount::import(in_scep).unwrap();
    let out_scep = account.export().unwrap();
    assert_eq!(
        account.display().as_str(),
        "f1jkzcn2xstealyngllhdjmeygrp6b5amvzhvklbi"
    );
    assert_eq!(out_scep.as_str(), in_scep);

    let secret_phrase = SecretPhrase::generate(Language::English,  MnemonicType::Words12);
    println!("secret_phrase: {}", secret_phrase.phrase());
    
    let account_secp = secret_phrase
        .derive_account(WalletType::Secp256k1, None)
        .unwrap();
    println!("Secp256k1 account: {}", account_secp.display());
    
    let account_bls = secret_phrase.derive_account(WalletType::Bls, None).unwrap();
    println!("Bls account: {}", account_bls.display());

}
