use std::str::FromStr;

use fvm_ipld_encoding::{Cbor, RawBytes};
use fvm_shared::{address::Address, econ::TokenAmount, message::Message};

pub(crate) fn string_to_tokenamount(value: &str) -> TokenAmount {
    let value = fvm_shared::bigint::BigInt::from_str(value).unwrap();
    TokenAmount::from_atto(value)
}

pub fn get_message_cid(
    from: String,
    to: String,
    nonce: u64,
    value: String,
    method: u64,
    params: String,
    gas_limit: i64,
    gas_fee_cap: String,
    gas_premium: String,
) -> String {
    let from = Address::from_str(&from).unwrap();
    let to = Address::from_str(&to).unwrap();
    let value = string_to_tokenamount(&value);
    let params = base64::decode(&params).unwrap();
    let params: RawBytes = params.into();
    let gas_fee_cap = string_to_tokenamount(&gas_fee_cap);
    let gas_premium = string_to_tokenamount(&gas_premium);

    let msg = Message {
        version: 0,
        from,
        to,
        sequence: nonce,
        value,
        method_num: method,
        params,
        gas_limit,
        gas_fee_cap,
        gas_premium,
    };

    let cid = msg.cid().unwrap();
    // base64::encode(cid)
    cid.to_string()
}

#[test]
fn test_get_message_cid() {
    let cid = get_message_cid(
        "t3sh7bfopxlxpaxhbrytc54qqwaeuytzlpfy36iuxjknjvm3ycj7ewajbnervggfoqwk4xhjdpvk54bpiesaya"
            .to_string(),
        "t03736".to_string(),
        11,
        "0".to_string(),
        4,
        "ggJA".to_string(),
        3044346,
        "100348".to_string(),
        "99294".to_string(),
    );
    assert_eq!(
        cid,
        "bafy2bzacebyorewi7uvs2g3dcsibpgkdjdncbu3xrk3n3ncws76esyywqwtog"
    );
}
