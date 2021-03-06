use std::fmt;
use serde::{ser, de, Serialize, Deserialize};
use super::bytes::Bytes;

/// KeyInfo is used for storing keys in KeyStore.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyInfo {
    /// The key type.
    pub r#type: KeyType,
    /// The private key corresponding to key type.
    pub private_key: Bytes,
}

#[derive(Eq, PartialEq, Clone)]
pub enum KeyType {
    /// secp256k1 key
    Secp256k1,
    /// bls key
    Bls,
    /// libp2p-host key
    Libp2pHost,
    /// jwt-hmac-secret key
    JwtHmacSecret,
    /// other key
    Other(String),
}

impl fmt::Debug for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyType::Secp256k1 => "secp256k1",
            KeyType::Bls => "bls",
            KeyType::Libp2pHost => "libp2p-host",
            KeyType::JwtHmacSecret => "jwt-hmac-secret",
            KeyType::Other(s) => s.as_str(),
        };
        f.write_str(s)
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

// Implement JSON serialization for KeyType.
impl ser::Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

// Implement JSON deserialization for KeyType.
impl<'de> de::Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "secp256k1" => KeyType::Secp256k1,
            "bls" => KeyType::Bls,
            "libp2p-host" => KeyType::Libp2pHost,
            "jwt-hmac-secret" => KeyType::JwtHmacSecret,
            _ => KeyType::Other(s),
        })
    }
}