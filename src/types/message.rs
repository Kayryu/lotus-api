use serde::{Deserialize, Serialize};
use num_bigint::{BigInt};
use super::utils::{bigint_json, bytes_json};
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
    #[serde(with = "bigint_json")]
    pub gas_limit: BigInt,
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

#[cfg(test)]
mod tests {

    #[test]
    fn message_receipt_json_test() {

    }
}