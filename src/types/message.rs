use serde::{Deserialize, Serialize};
use num_bigint::{BigInt};
use cid::Cid;
use super::utils::{bigint_json, bytes_json, vec_cid_json, cid_json};
use super::crypto::Signature;
use super::address::Address;
use super::bytes::Bytes;

/// The signed message (a message with signature).
#[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SignedMessage {
    /// The unsigned message.
    pub message: UnsignedMessage,
    /// The signature.
    pub signature: Signature,
}

/// The unsigned message.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnsignedMessage {
    ///
    pub version: i64,
    /// The receiver of the unsigned message.
    pub to: Address,
    /// The sender of the unsigned message.
    pub from: Address,
    /// The nonce.
    pub nonce: u64,
    /// The value.
    #[serde(with = "bigint_json")]
    pub value: BigInt,
    /// The price of gas.
    #[serde(with = "bigint_json")]
    pub gas_price: BigInt,
    /// The limit of gas.
   // #[serde(with = "bigint_json")]
    pub gas_limit: u64,
    /// The method.
    pub method: u64,
    /// The params of method.
//    #[serde(with = "bytes")]
    pub params: Bytes,
}

/// The receipt of applying message.
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageReceipt {
    /// The exit code of VM.
    pub exit_code: u8,
    /// The return bytes.
    #[serde(with = "bytes_json")]
    pub r#return: Vec<u8>,
    /// The used number of gas.
    #[serde(with = "bigint_json")]
    pub gas_used: BigInt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockMessages {
    pub bls_messages: Vec<UnsignedMessage>,
    pub secpk_messages: Vec<SignedMessage>,
    #[serde(with = "vec_cid_json")]
    pub cids: Vec<Cid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParentMessage {
    #[serde(with = "cid_json")]
    pub cid: Cid,
    pub message: UnsignedMessage,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjStat {
    pub size: u64,
    pub links: u64,
}

#[cfg(test)]
mod tests {

    #[test]
    fn message_receipt_json_test() {

    }
}