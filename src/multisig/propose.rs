use std::str::FromStr;

use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::MethodNum;
/// Propose method call parameters.
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ProposeParams {
    pub to: Address,
    pub value: TokenAmount,
    pub method: MethodNum,
    pub params: RawBytes,
}

// return params string
pub fn propose_multisig_params(
    method: u64,
    _params: Option<String>,
    to: String,
    value: Option<String>,
) -> String {
    let value = if let Some(v) = value {
        v
    } else {
        "0".to_string()
    };

    let to = fvm_shared::address::Address::from_str(&to).unwrap();
    // let value = BigInt::from_str(&value).unwrap();
    let t = fvm_shared::bigint::BigInt::from_str(&value).unwrap();
    let value = TokenAmount::from_atto(t);

    let params = ProposeParams {
        to,
        value,
        method,
        params: vec![].into(),
    };

    let t = fvm_ipld_encoding::to_vec(&params).unwrap();
    base64::encode(t)
}

#[test]
fn test_multisig_propose_params() {
    let method = 0;
    let params = None;
    let to = "t1jkzcn2xstealyngllhdjmeygrp6b5amvzhvklbi".to_string();
    let value = "10000000000000000000".to_string();

    let cbor = propose_multisig_params(method, params, to, Some(value));
    assert_eq!(&cbor, "hFUBSrIm6vKZALw0y1nGlhMGi/wegZVJAIrHIwSJ6AAAAEA=");
}
