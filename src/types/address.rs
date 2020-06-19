use serde::{de, ser};

pub(crate) const NETWORK_MAINNET_PREFIX: &str = "f";
pub(crate) const NETWORK_TESTNET_PREFIX: &str = "t";

/// The network type used by the address.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Network {
    /// Main network, prefix: 'f'.
    Main,
    /// Test network, prefix: 't'.
    Test,
}

impl Default for Network {
    fn default() -> Self {
        Network::Test
    }
}

impl Network {
    /// Return the prefix identifier of network.
    pub fn prefix(self) -> &'static str {
        match self {
            Network::Main => NETWORK_MAINNET_PREFIX,
            Network::Test => NETWORK_TESTNET_PREFIX,
        }
    }
}


/// Protocol Identifier.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Protocol {
    /// `ID` protocol, identifier: 0.
    Id = 0,
    /// `Secp256k1` protocol, identifier: 1.
    Secp256k1 = 1,
    /// `Actor` protocol, identifier: 2.
    Actor = 2,
    /// `BLS` protocol, identifier: 3.
    Bls = 3,
}

/// The general address structure.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Address {
    // `ID` protocol: payload is VarInt encoding.
    // `Secp256k1` protocol: payload is the hash of pubkey (length = 20)
    // `Actor` protocol: payload length = 20
    // `BLS` protocol: payload is pubkey (length = 48)
    protocol: Protocol,
    payload: Vec<u8>,
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || s.len() > constant::MAX_ADDRESS_STRING_LEN {
            return Err(AddressError::InvalidLength);
        }

        match &s[0..1] {
            NETWORK_MAINNET_PREFIX | NETWORK_TESTNET_PREFIX => {
                if &s[0..1] != NETWORK_DEFAULT.prefix() {
                    return Err(AddressError::MismatchNetwork);
                }
            }
            _ => return Err(AddressError::UnknownNetwork),
        }

        let protocol = match &s[1..2] {
            "0" => Protocol::Id,
            "1" => Protocol::Secp256k1,
            "2" => Protocol::Actor,
            "3" => Protocol::Bls,
            _ => return Err(AddressError::UnknownProtocol),
        };

        let raw = &s[2..];

        match protocol {
            Protocol::Id => {
                if raw.len() > constant::MAX_U64_LEN {
                    return Err(AddressError::InvalidLength);
                }
                match raw.parse::<u64>() {
                    Ok(id) => Self::new_id_addr(id),
                    Err(_) => Err(AddressError::InvalidPayload),
                }
            }
            Protocol::Secp256k1 => Self::new_with_check(
                Protocol::Secp256k1,
                raw.as_bytes(),
                constant::PAYLOAD_HASH_LEN,
            ),
            Protocol::Actor => {
                Self::new_with_check(Protocol::Actor, raw.as_bytes(), constant::PAYLOAD_HASH_LEN)
            }
            Protocol::Bls => {
                Self::new_with_check(Protocol::Bls, raw.as_bytes(), constant::BLS_PUBLIC_KEY_LEN)
            }
        }
    }
}

// Implement JSON serialization for Address.
impl ser::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

// Implement JSON deserialization for Address.
impl<'de> de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        let addr = String::deserialize(deserializer)?;
        addr.parse::<Address>().map_err(de::Error::custom)
    }
}