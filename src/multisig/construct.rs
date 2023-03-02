use std::str::FromStr;

use cid::Cid;
use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::RawBytes;
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ExecParams {
    pub code_cid: Cid,
    pub constructor_params: RawBytes,
}

use fil_actor_multisig_v9::ConstructorParams;
use fvm_shared::{address::Address, clock::ChainEpoch};

// return params string
pub fn create_multisig_params(
    addresses: Vec<String>,
    threshold: u64,
    unlock_duration: i64,
    start_epoch: i64,
) -> String {
    let signers = addresses
        .iter()
        .map(|t| Address::from_str(t).unwrap())
        .collect();

    let unlock_duration: ChainEpoch = unlock_duration;
    let start_epoch: ChainEpoch = start_epoch;

    let msig_params = ConstructorParams {
        signers,
        num_approvals_threshold: threshold,
        unlock_duration,
        start_epoch,
    };
    let msig_params_bytes = fvm_ipld_encoding::to_vec(&msig_params).unwrap();

    let cid: cid::Cid = "bafk2bzacec6gmi7ucukr3bk67akaxwngohw3lsg3obvdazhmfhdzflkszk3tg"
        .to_string()
        .parse()
        .unwrap();

    let params = ExecParams {
        code_cid: cid,
        constructor_params: msig_params_bytes.into(),
    };

    let t = fvm_ipld_encoding::to_vec(&params).unwrap();
    base64::encode(t)
}
