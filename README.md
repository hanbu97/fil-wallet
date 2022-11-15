# Flair-wallet
Flair-wallet is wallet releated part of our actively developing tool Flair for filecoin infrastructure.

## Note: still under develop. DO NOT USE IN PRODUCTION!

Filecoin wallet can be generated from bls and secp256k1 algorithm. We implement account generation and import/export for both methods. 

Secp256k1 part is inspired from crypto-wallet-gen, and reuse some of its functions. Bls part is inspired from ChainSafe's bls implemention(ts and rust).

Besides, we add support to mnemonic generation and create a new account from mnemonic.

``` rust
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











