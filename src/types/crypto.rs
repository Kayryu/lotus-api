use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureType {
    /// The `Secp256k1` signature.
    Secp256k1 = 1,
    /// The `BLS` signature.
    Bls = 2,
}

impl Default for SignatureType {
    fn default() -> Self {
        SignatureType::Bls
    }
}

impl TryFrom<u8> for SignatureType {
    type Error = CryptoError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SignatureType::Secp256k1),
            2 => Ok(SignatureType::Bls),
            _ => Err(CryptoError::UnknownSignatureType(value)),
        }
    }
}

impl From<SignatureType> for u8 {
    fn from(ty: SignatureType) -> Self {
        match ty {
            SignatureType::Secp256k1 => 1,
            SignatureType::Bls => 2,
        }
    }
}

/// The general signature structure.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signature {
    /// The signature type.
    r#type: SignatureType,
    /// Tha actual signature bytes.
    /// secp256k1: signature (64 bytes) + recovery_id (1 byte)
    /// bls: signature (96 bytes)
    #[serde(with = "plum_bytes")]
    data: Vec<u8>,
}

