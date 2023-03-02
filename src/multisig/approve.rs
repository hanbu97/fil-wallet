use fvm_ipld_encoding::serde_bytes;
use fvm_ipld_encoding::tuple::*;
use serde::{Deserialize, Serialize};

/// Transaction ID type
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TxnID(pub i64);

#[derive(Clone, PartialEq, Eq, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct TxnIDParams {
    pub id: TxnID,
    /// Optional hash of proposal to ensure an operation can only apply to a
    /// specific proposal.
    #[serde(with = "serde_bytes")]
    pub proposal_hash: Vec<u8>,
}

// return params string
pub fn approve_multisig_params(txnid: i64) -> String {
    let params = TxnIDParams {
        id: TxnID(txnid),
        proposal_hash: vec![],
    };

    let t = fvm_ipld_encoding::to_vec(&params).unwrap();
    base64::encode(t)
}

#[test]
fn test_multisig_approve_params() {
    let cbor = approve_multisig_params(0);
    dbg!(&cbor);
}
