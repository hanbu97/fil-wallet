use std::str::FromStr;

use fil_actor_init_v9::ExecParams;
use fvm_shared::{address::Address, clock::ChainEpoch};

use fil_actor_multisig_v9::ConstructorParams;

pub fn create_multisig_constructor_params(addresses: Vec<String>) -> String {
    let signers = addresses
        .iter()
        .map(|t| Address::from_str(t).unwrap())
        .collect();

    let unlock_duration: ChainEpoch = 0;
    let start_epoch: ChainEpoch = 0;

    let msig_params = ConstructorParams {
        signers,
        num_approvals_threshold: 2,
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
