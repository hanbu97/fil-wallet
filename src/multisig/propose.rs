use std::str::FromStr;

use fil_actor_init_v9::ExecParams;
use fil_actor_multisig_v9::{ConstructorParams, ProposeParams};
use fvm_shared::{address::Address, clock::ChainEpoch, econ::TokenAmount};
use num_bigint::BigInt;

// return params string
pub fn multisig_propose_params(
    method: u64,
    params: Option<String>,
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

    let cbor = multisig_propose_params(method, params, to, Some(value));
    assert_eq!(&cbor, "hFUBSrIm6vKZALw0y1nGlhMGi/wegZVJAIrHIwSJ6AAAAEA=");
}
