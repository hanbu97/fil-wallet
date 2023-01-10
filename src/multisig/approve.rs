use fil_actor_multisig_v9::TxnIDParams;

// return params string
pub fn approve_multisig_params(txnid: i64) -> String {
    let params = TxnIDParams {
        id: fil_actor_multisig_v9::TxnID(txnid),
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
