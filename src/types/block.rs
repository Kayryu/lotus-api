use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    /// The block header.
    pub header: BlockHeader,
    /// The `BLS` messages.
    pub bls_messages: Vec<UnsignedMessage>,
    /// The `Secp256k1` messages.
    pub secpk_messages: Vec<SignedMessage>,
}
