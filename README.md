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

// generate from random
let secret_phrase = SecretPhrase::generate(Language::English,  MnemonicType::Words12);
println!("secret_phrase: {}", secret_phrase.phrase());

let account_secp = secret_phrase
    .derive_account(WalletType::Secp256k1, None)
    .unwrap();
println!("Secp256k1 account: {}", account_secp.display());

let account_bls = secret_phrase.derive_account(WalletType::Bls, None).unwrap();
println!("Bls account: {}", account_bls.display());


// generate from given secret phrase
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
```
```
secret_phrase: betray ribbon visit topple release angle inspire soul private bottom face buddy
Secp256k1 account: f1ihd67zlxq6zbvbtnox4xsjn6htps34ys3mfjv6a
Bls account: f3uvn2j4mgp2tz3oiinh3jwnepy6zhhsd76ngec6dgwf5e2wdnfl3ylu7bgqa6jkbqv3e4mua6ectzbleflmtq
Secp256k1 account: f1ihd67zlxq6zbvbtnox4xsjn6htps34ys3mfjv6a
Bls account: f3uvn2j4mgp2tz3oiinh3jwnepy6zhhsd76ngec6dgwf5e2wdnfl3ylu7bgqa6jkbqv3e4mua6ectzbleflmtq

```

For the key derivation part of bls(f3) account, is based on https://iancoleman.io/eip2333/ , making our derived secret key the same. But still have invalid keys, when invalid key occurs, we increase the last index of derive path(use index) to find the first valid secret key for bls signature.

### Signature
``` rust
let in_bls = "7b2254797065223a22626c73222c22507269766174654b6579223a2270657341657756666d382f6f7a574c736b6f767a7464677a62566d73677657695a70506f346d53367269493d227d";
let account = FlairAccount::import(in_bls).unwrap();
let cid_string = "bafy2bzacec6m3lsogelnttnn4ck7dr35zpyuynqaliiqycx4zraqmqmjebc36".to_string();

let signature = account.sign(cid_string).unwrap();
assert_eq!("iKKZhRQkg2qQ8fWWIMFFHjJSAWaOgIkvVSAJ4aZ0pL2fPeMOsSzOoqqfE9mj/qtCDAfeDWCgvVzTwVvSNNt5jL+LPwSSGinfHtO01urO0vuOq2EUCVcgH57ftzf5SAJT",&signature);

```

### multisig
- construct params
``` rust
let addresses = vec![
    "t3quwa4vrjyk77zqbpeball2xlemlezcdfolexqki4ialamvql7eap2u6ryzc4ufzeuyplti5ruopbtansv63q".to_string(),
    "t3qyqntzkarnpzg66gcgotopmducfqfvvhg7ee7l6ral5xbzimhf5qrduufsxemulrb2zfjdmpdvftaljzuhva".to_string(),
    "t3sh7bfopxlxpaxhbrytc54qqwaeuytzlpfy36iuxjknjvm3ycj7ewajbnervggfoqwk4xhjdpvk54bpiesaya".to_string(),
];
let cbor = flair_wallet::create_multisig_params(addresses);
assert_eq!(
    r#"gtgqWCcAAVWg5AIgvGYj9BUVHYVe+BQL2aZx7bXI23BqMGTsKceSrVLKtzNYnoSDWDEDhSwOVinCv/zALyBAterrIxZMiGVyyXgpHEAWBlYL+QD9U9HGRcoXJKYeuaOxo54ZWDEDhiDZ5UCLX5N7xhGdNz2DoIsC1qc3yE+v0QL7cOUMOXsIjpQsrkZRcQ6yVI2PHUswWDEDkf4Sufdd3gucMcTF3kIWASmJ5W8uN+RS6VNTVm8CT8lgJC0kamMV0LK5c6RvqrvAAgAA"#,
    &cbor
);
```

## Todo
- Remove unnecessary dependencies
- add more tests and use cases
- improve ergonomic
- audit

## Issue
Welcome.











