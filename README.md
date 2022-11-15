# Flair-wallet
Flair-wallet is wallet releated part of our actively developing tool Flair for filecoin infrastructure.

## Note: still under development. DO NOT USE IN PRODUCTION!

Filecoin wallet can be generated from bls and secp256k1 algorithm. We implement account generation and import/export for both methods. 

Secp256k1 part is inspired from crypto-wallet-gen, and reuse some of its functions. Bls part is inspired from ChainSafe's bls implemention(ts and rust).

``` rust
use flair_wallet::FlairAccount;

let in_bls = "7b2254797065223a22626c73222c22507269766174654b6579223a226b434b523969566b73615a6672746b513979356e3269615862317279766d314d37637357456352313142673d227d";
let account = FlairAccount::import(in_bls).unwrap();
let out_bls = account.export().unwrap();
assert_eq!(account.display().as_str(), "f3vtr3myof7qghg3eq75qfagbfemrhycsqypmz2t6pfwwq5m3w73wlthhbtlrnjpyjxnhwz5pxboonb3bliv6q");
assert_eq!(out_bls.as_str(), in_bls);

let in_scep = "7b2254797065223a22736563703235366b31222c22507269766174654b6579223a2235734d384f2b6639554161686d78726d61653533776a667056374338664b6b426c414c4c44366e717a666b3d227d";
let account = FlairAccount::import(in_scep).unwrap();
let out_scep = account.export().unwrap();
assert_eq!(account.display().as_str(), "f1jkzcn2xstealyngllhdjmeygrp6b5amvzhvklbi");
assert_eq!(out_scep.as_str(), in_scep);
```

Besides, we add support to mnemonic generation and create a new account from mnemonic.

``` rust
use bip39::{Language, MnemonicType};
use flair_wallet::{SecretPhrase, WalletType};

let secret_phrase = SecretPhrase::generate(Language::English,  MnemonicType::Words12);
println!("secret_phrase: {}", secret_phrase.phrase());

let account_secp = secret_phrase
    .derive_account(WalletType::Secp256k1, None)
    .unwrap();
println!("Secp256k1 account: {}", account_secp.display());

let account_bls = secret_phrase.derive_account(WalletType::Bls, None).unwrap();
println!("Bls account: {}", account_bls.display());

```
```
secret_phrase: betray ribbon visit topple release angle inspire soul private bottom face buddy
Secp256k1 account: f1ihd67zlxq6zbvbtnox4xsjn6htps34ys3mfjv6a
Bls account: f3uvn2j4mgp2tz3oiinh3jwnepy6zhhsd76ngec6dgwf5e2wdnfl3ylu7bgqa6jkbqv3e4mua6ectzbleflmtq
```

For the key derivation part of bls(f3) account, is based on https://iancoleman.io/eip2333/ , making our derived secret key the same. But still have invalid keys, when invalid key occurs, we increase the last index of derive path(use index) to find the first valid secret key for bls signature.

## Todo
- Remove unnecessary dependencies
- add more tests and use cases
- improve ergonomic
- audit

## Issue
Welcome.











