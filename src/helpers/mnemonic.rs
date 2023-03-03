use bip39::{Mnemonic, Seed};
// use crypto::{digest::Digest, sha2::Sha256};
use crypto_wallet_gen::{Bip39Mnemonic, Bip44DerivationPath, MnemonicFactory};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num, Pow};
use sha2::{Digest, Sha256};

use crate::{
    helpers::accounts::generate_account_from_private, types::WalletType, FlairAccount, FlairPrivate,
};

const DIGEST_SIZE: usize = 32;
const NUM_DIGESTS: usize = 255;
const OUTPUT_SIZE: usize = DIGEST_SIZE * NUM_DIGESTS;

pub fn derive_master_key(seed: &[u8]) -> anyhow::Result<Vec<u8>> {
    if seed.len() < 32 {
        return Err(anyhow::anyhow!("seed must be >= 32 bytes"));
    }
    Ok(hkdf_mod_r(seed))
}

fn hkdf(ikm: &[u8], salt: &[u8], info: &[u8], okm: &mut [u8]) {
    use hkdf::Hkdf;
    let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
    hk.expand(&info, okm).unwrap();
}

pub fn hkdf_mod_r(ikm: &[u8]) -> Vec<u8> {
    let mut ikm = ikm.to_vec();
    ikm.push(0);

    let salt = "BLS-SIG-KEYGEN-SALT-".as_bytes().to_vec();
    let mut hasher = Sha256::new();
    hasher.reset();
    hasher.update(&salt);
    // let mut result: Vec<u8> = vec![0; hasher.output_bytes()];
    // hasher.result(&mut result);

    let result = hasher.finalize();
    let salt = result;
    // let salt = &result[..];

    let mut okm = [0u8; 48];
    hkdf(&ikm, &salt, &[0, 48], &mut okm);

    let r = BigUint::from_str_radix(
        "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001",
        16,
    )
    .unwrap();
    let out = BigUint::from_bytes_be(okm.as_ref()) % r;

    out.to_bytes_be()
}

fn ikm_to_lamport_sk(ikm: &[u8], salt: &[u8], split_bytes: &mut [[u8; DIGEST_SIZE]; NUM_DIGESTS]) {
    let mut okm = [0u8; OUTPUT_SIZE];
    hkdf(ikm, salt, &[], &mut okm);
    for r in 0..NUM_DIGESTS {
        split_bytes[r].copy_from_slice(&okm[r * DIGEST_SIZE..(r + 1) * DIGEST_SIZE])
    }
}

fn flip_bits(num: &[u8]) -> BigUint {
    let num = BigUint::from_bytes_be(num);
    num ^ (Pow::pow(
        &BigUint::from_u64(2).unwrap(),
        &BigUint::from_u64(256).unwrap(),
    ) - &BigUint::from_u64(1).unwrap())
}

fn parent_sk_to_lamport_pk(ikm: &[u8], index: u32) -> Vec<u8> {
    let salt = index.to_be_bytes();

    let mut lamport_0 = [[0u8; DIGEST_SIZE]; NUM_DIGESTS];
    ikm_to_lamport_sk(ikm, salt.as_slice(), &mut lamport_0);

    let not_ikm = flip_bits(ikm).to_bytes_be();

    let mut lamport_1 = [[0u8; DIGEST_SIZE]; NUM_DIGESTS];
    ikm_to_lamport_sk(not_ikm.as_slice(), salt.as_slice(), &mut lamport_1);

    let mut combined = [[0u8; DIGEST_SIZE]; NUM_DIGESTS * 2];
    combined[..NUM_DIGESTS].clone_from_slice(&lamport_0[..NUM_DIGESTS]);
    combined[NUM_DIGESTS..NUM_DIGESTS * 2].clone_from_slice(&lamport_1[..NUM_DIGESTS]);

    let mut flattened_key = [0u8; OUTPUT_SIZE * 2];
    for i in 0..NUM_DIGESTS * 2 {
        let mut sha256 = Sha256::new();
        let sha_slice = combined[i].clone();
        // sha256.input(sha_slice);
        // sha256.result(sha_slice);

        sha256.update(sha_slice);
        let result = sha256.finalize();
        combined[i].clone_from_slice(&result[..DIGEST_SIZE]);

        flattened_key[i * DIGEST_SIZE..(i + 1) * DIGEST_SIZE].clone_from_slice(&result);
    }
    let mut sha256 = Sha256::new();
    // sha256.input(&flattened_key);
    sha256.update(&flattened_key);
    // let cmp_pk: &mut [u8] = &mut [0u8; DIGEST_SIZE];
    // sha256.result(cmp_pk);
    // cmp_pk.to_vec()
    sha256.finalize().to_vec()
}

pub fn derive_child_sk(ikm: &[u8], index: u32) -> Vec<u8> {
    let key = parent_sk_to_lamport_pk(ikm, index);
    hkdf_mod_r(&key)
}

// EIP 2334
pub fn path_to_node(path: &str) -> anyhow::Result<Vec<u32>> {
    let mut parsed: Vec<&str> = path.split('/').collect();
    let m = parsed.remove(0);
    if m != "m" {
        return Err(anyhow::anyhow!("First value must be m, got {}", m));
    }

    let mut ret = vec![];
    for value in parsed {
        match value.parse::<u32>() {
            Ok(v) => ret.push(v),
            Err(_) => return Err(anyhow::anyhow!("could not parse value: {}", value)),
        }
    }

    Ok(ret)
}

pub fn drive_from_path(ikm: &[u8], path: &str) -> anyhow::Result<Vec<u8>> {
    let indexes = path_to_node(path)?;
    let mut key = ikm.to_vec();

    for index in indexes {
        key = derive_child_sk(&key, index);
    }

    Ok(key)
}

pub fn gen_account(phrase: &str, password: &str) -> anyhow::Result<FlairAccount> {
    use crypto_wallet_gen::Mnemonic;
    let mnemonic = Bip39Mnemonic::from_phrase(phrase)?;
    let account_index = 0;
    let change_index: Option<u32> = Some(0);
    let address_index: Option<u32> = Some(0);

    let master_key = mnemonic.to_private_key(password)?;
    let derivation_path = Bip44DerivationPath {
        coin_type: crypto_wallet_gen::CoinType::FIL,
        account: account_index,
        change: change_index,
        address_index,
    };

    // Secp256k1
    let derived = master_key.derive(derivation_path)?;
    let private_key: FlairPrivate = derived.key_part().into_bytes().into();
    let account = generate_account_from_private(&WalletType::Secp256k1, &private_key)?;

    Ok(account)
}

pub fn gen_account_bls(mnemonic: &Mnemonic, password: &str) -> anyhow::Result<FlairAccount> {
    let seed = Seed::new(mnemonic, password);
    let seed_bytes = seed.as_bytes();

    let mut idx = 0;

    // add use index until success
    // as eip-2333 shows "more than 54% of keys generated by BIP32 would be invalid"
    loop {
        let master_key = derive_master_key(seed_bytes)?;
        let key = drive_from_path(&master_key, &format!("m/12381/461/0/{idx}"))?;

        let mut private_key: FlairPrivate = key.into();
        private_key.set_type(WalletType::Bls);

        match generate_account_from_private(&WalletType::Bls, &private_key) {
            Ok(a) => break Ok(a),
            Err(_) => idx += 1,
        }
    }
}

#[test]
fn test_derive_master_key() {
    let phrase = "poverty fury pencil useful catch turn nation select bid fashion need intact";
    let mnemonic = Mnemonic::from_phrase(&phrase, bip39::Language::English).unwrap();
    let seed = Seed::new(&mnemonic, "");
    let seed_bytes = seed.as_bytes();

    let master_key = derive_master_key(seed_bytes).unwrap();
    let key = drive_from_path(&master_key, "m/12381/461/0/0").unwrap();

    let hex_key = hex::encode(&key);
    // according to results from https://iancoleman.io/eip2333/
    assert_eq!(
        hex_key.as_str(),
        "4bfbf5bfbc86ebbcd71d183441d2cebed2759de45401863b3fe1c7293ca1f56f"
    );

    let mut private_key: FlairPrivate = key.into();
    private_key.set_type(WalletType::Bls);

    let account = generate_account_from_private(&WalletType::Bls, &private_key).unwrap();

    println!("{}", account.display());
}
