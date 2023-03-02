use blake2b_simd::Params;
use bls_signatures::{PrivateKey as BlsPrivate, Serialize as BlsSerialize};
use fvm_shared::address::Address;
use libsecp256k1::{Message as SecpMessage, SecretKey as SecpPrivate};
use serde::{Deserialize, Serialize};

pub fn blake2b_256(ingest: &[u8]) -> [u8; 32] {
    let digest = Params::new()
        .hash_length(32)
        .to_state()
        .update(ingest)
        .finalize();

    let mut ret = [0u8; 32];
    ret.clone_from_slice(digest.as_bytes());
    ret
}

use crate::{
    helpers::accounts::{
        generate_account, generate_account_from_encoded_string, generate_account_from_private,
        generate_account_from_public,
    },
    types::WalletType,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct FlairPrivate {
    #[serde(rename = "Type")]
    wallet_type: String,
    #[serde(rename = "PrivateKey")]
    private_key: String,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl From<Vec<u8>> for FlairPrivate {
    fn from(d: Vec<u8>) -> Self {
        Self {
            wallet_type: "secp256k1".to_string(),
            private_key: base64::encode(&d),
            data: d,
        }
    }
}
impl FlairPrivate {
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn set_type(&mut self, wallet_type: WalletType) {
        self.wallet_type = wallet_type.to_string()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FlairPublic(Vec<u8>);
impl From<Vec<u8>> for FlairPublic {
    fn from(d: Vec<u8>) -> Self {
        Self(d)
    }
}

impl From<FlairPublic> for Vec<u8> {
    fn from(t: FlairPublic) -> Self {
        t.0
    }
}

impl FlairPublic {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct FlairAddress(Address);
impl From<Address> for FlairAddress {
    fn from(a: Address) -> Self {
        Self(a)
    }
}

#[derive(Debug)]
pub struct FlairAccount {
    wallet_type: WalletType,
    pub private: Option<FlairPrivate>,
    public: Option<FlairPublic>,
    address: FlairAddress,
}

impl FlairAccount {
    /// set public key
    pub(crate) fn set_public(&mut self, public_key: FlairPublic) {
        self.public = Some(public_key)
    }

    /// set private key
    pub(crate) fn set_private(&mut self, private_key: FlairPrivate) {
        self.private = Some(private_key)
    }

    /// get account inner address
    pub fn get_address(&self) -> FlairAddress {
        self.address
    }

    /// get wallet type
    pub fn get_type(&self) -> WalletType {
        self.wallet_type
    }

    /// create FlairAccount from address
    pub fn from_address(wallet_type: WalletType, address: FlairAddress) -> Self {
        Self {
            wallet_type,
            private: None,
            public: None,
            address,
        }
    }

    /// create FlairAccount from public key
    pub fn from_public(wallet_type: WalletType, public_key: FlairPublic) -> anyhow::Result<Self> {
        generate_account_from_public(&wallet_type, &public_key)
    }

    /// create FlairAccount from private key
    pub fn from_private(
        wallet_type: WalletType,
        private_key: FlairPrivate,
    ) -> anyhow::Result<Self> {
        generate_account_from_private(&wallet_type, &private_key)
    }

    /// create FlairAccount from private key
    pub fn generate(wallet_type: WalletType) -> anyhow::Result<Self> {
        generate_account(&wallet_type)
    }

    /// get wallet address
    pub fn display(&self) -> String {
        self.address.0.to_string()
    }

    /// export filecoin format private key
    pub fn export(&self) -> anyhow::Result<String> {
        if let Some(ref prv) = self.private {
            let key_json = serde_json::to_string(prv)?;

            let out = hex::encode(key_json);
            Ok(out)
        } else {
            Err(anyhow::anyhow!("Private Key not found!"))
        }
    }

    /// import filecoin format private key
    pub fn import(key: &str) -> anyhow::Result<Self> {
        generate_account_from_encoded_string(key)
    }

    /// sign message
    pub fn sign(&self, cid: String) -> anyhow::Result<String> {
        match &self.private {
            Some(private_key) => {
                let private_key = private_key.to_vec();
                let cid: cid::Cid = cid.parse()?;
                let msg = cid.to_bytes().to_vec();

                match self.wallet_type {
                    WalletType::Bls => {
                        let priv_key = BlsPrivate::from_bytes(&private_key)?;
                        let sig = priv_key.sign(&msg).as_bytes();
                        let out = base64::encode(sig);
                        Ok(out)
                    }
                    WalletType::Secp256k1 => {
                        let priv_key = SecpPrivate::parse_slice(&private_key)?;
                        let msg_hash = blake2b_256(&msg);
                        let message = SecpMessage::parse(&msg_hash);
                        let (sig, recovery_id) = libsecp256k1::sign(&message, &priv_key);
                        let mut new_bytes = [0; 65];
                        new_bytes[..64].copy_from_slice(&sig.serialize());
                        new_bytes[64] = recovery_id.serialize();

                        let out = new_bytes.to_vec();
                        let out = base64::encode(out);
                        Ok(out)
                    }
                }
            }
            None => Err(anyhow::anyhow!("Not Authourized Account")),
        }
    }
}

mod test {
    // #[test]
    // fn test_account_from_encoded_bls_private() {
    //     let key = "SWIIbklHT09OwCJ2SnHq57RBLIJc5VtWsM3+SGZ+S7I=";
    //     let private_key: super::FlairPrivate = base64::decode(key).unwrap().into();
    //     let account = crate::helpers::accounts::generate_account_from_private(
    //         &crate::types::WalletType::Bls,
    //         &private_key,
    //     )
    //     .unwrap();
    //     dbg!(account.display());
    // }
    #[test]
    fn test_account_import_export() {
        let in_bls = "7b2254797065223a22626c73222c22507269766174654b6579223a226b434b523969566b73615a6672746b513979356e3269615862317279766d314d37637357456352313142673d227d";
        let account = super::FlairAccount::import(in_bls).unwrap();
        let out_bls = account.export().unwrap();
        assert_eq!(account.display().as_str(), "f3vtr3myof7qghg3eq75qfagbfemrhycsqypmz2t6pfwwq5m3w73wlthhbtlrnjpyjxnhwz5pxboonb3bliv6q");
        assert_eq!(out_bls.as_str(), in_bls);

        let in_scep = "7b2254797065223a22736563703235366b31222c22507269766174654b6579223a2235734d384f2b6639554161686d78726d61653533776a667056374338664b6b426c414c4c44366e717a666b3d227d";
        let account = super::FlairAccount::import(in_scep).unwrap();
        let out_scep = account.export().unwrap();
        dbg!(account.display());
        assert_eq!(out_scep.as_str(), in_scep);

        let in_bls = "7b2254797065223a22626c73222c22507269766174654b6579223a226b434b523969566b73615a6672746b513979356e3269615862317279766d314d37637357456352313142673d227d";
        let account = super::FlairAccount::import(in_bls).unwrap();
        let out_bls = account.export().unwrap();
        assert_eq!(account.display().as_str(), "f3vtr3myof7qghg3eq75qfagbfemrhycsqypmz2t6pfwwq5m3w73wlthhbtlrnjpyjxnhwz5pxboonb3bliv6q");
        assert_eq!(out_bls.as_str(), in_bls);
    }

    #[test]
    fn test_account_import_export1() {
        let in_scep = "7b2254797065223a6e756c6c2c22507269766174654b6579223a22674935796e756f61442b695a6534336b556a32454947594d57715462516d4b682f59632b38572b494570383d227d";
        let account = super::FlairAccount::import(in_scep).unwrap();
        dbg!(account.display());
    }

    #[test]
    fn test_account_import_export2() {
        let in_scep = "7b2254797065223a22626c73222c22507269766174654b6579223a225a55674656777a6f6d4335625143454e6c7a47366a6b7448384b456f5079457241624745685965656b686b3d227d";
        let account = super::FlairAccount::import(in_scep).unwrap();
        dbg!(account.display());
    }
}
