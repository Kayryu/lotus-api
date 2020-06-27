use serde::{de, ser};
use thiserror::Error;
use std::str::FromStr;
use super::constants::*;

pub static NETWORK_DEFAULT:Network = Network::Main;

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

/// Errors generated from this library.
#[derive(PartialEq, Eq, Debug, Error)]
pub enum AddressError {
    /// Unknown network.
    #[error("unknown network")]
    UnknownNetwork,
    /// Mismatch network.
    #[error("Network do not match default network (current: {})", NETWORK_DEFAULT.prefix())]
    MismatchNetwork,
    /// Unknown address protocol.
    #[error("unknown protocol")]
    UnknownProtocol,
    /// Invalid address payload.
    #[error("invalid address payload")]
    InvalidPayload,
    /// Invalid address length.
    #[error("invalid address length")]
    InvalidLength,
    /// Invalid address checksum.
    #[error("invalid address checksum")]
    InvalidChecksum,
    /// Base32 decode error.
    #[error("base32 decode error: {0}")]
    Base32Decode(#[from] data_encoding::DecodeError),
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

impl Address {
    /// Create an address with the given protocol and payload
    pub(crate) fn new<T: Into<Vec<u8>>>(
        protocol: Protocol,
        payload: T,
    ) -> Result<Self, AddressError> {
        let payload = payload.into();
        match protocol {
            Protocol::Id => {}
            Protocol::Secp256k1 | Protocol::Actor => {
                if payload.len() != constant::PAYLOAD_HASH_LEN {
                    return Err(AddressError::InvalidPayload);
                }
            }
            Protocol::Bls => {
                if payload.len() != constant::BLS_PUBLIC_KEY_LEN {
                    return Err(AddressError::InvalidPayload);
                }
            }
        }

        Ok(Self { protocol, payload })
    }

    /// Create an address using the `Id` protocol.
    pub fn new_id_addr(id: u64) -> Result<Self, AddressError> {
        let mut payload_buf = unsigned_varint::encode::u64_buffer();
        let payload = unsigned_varint::encode::u64(id, &mut payload_buf);
        Self::new(Protocol::Id, payload)
    }

    /// Create an address using the `Secp256k1` protocol.
    pub fn new_secp256k1_addr(pubkey: &[u8]) -> Result<Self, AddressError> {
        if pubkey.len() != constant::SECP256K1_FULL_PUBLIC_KEY_LEN
            && pubkey.len() != constant::SECP256K1_RAW_PUBLIC_KEY_LEN
            && pubkey.len() != constant::SECP256K1_COMPRESSED_PUBLIC_KEY_LEN
        {
            return Err(AddressError::InvalidPayload);
        }
        Self::new(Protocol::Secp256k1, address_hash(pubkey))
    }

    /// Create an address using the `Actor` protocol.
    pub fn new_actor_addr(data: &[u8]) -> Result<Self, AddressError> {
        Self::new(Protocol::Actor, address_hash(data))
    }

    /// Create an address using the `BLS` protocol.
    pub fn new_bls_addr(pubkey: &[u8]) -> Result<Self, AddressError> {
        Self::new(Protocol::Bls, pubkey)
    }

    /// Create an address represented by the encoding bytes `addr` (protocol + payload).
    pub fn new_from_bytes(addr: &[u8]) -> Result<Self, AddressError> {
        if addr.len() <= 1 {
            return Err(AddressError::InvalidLength);
        }
        let protocol = Protocol::try_from(addr[0])?;
        Self::new(protocol, &addr[1..])
    }

    /// Return the network type of the address.
    pub fn network(&self) -> Network {
        **NETWORK_DEFAULT
    }

    /// Return the protocol of the address.
    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Return the payload of the address.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// If the `Address` is an ID address, return the ID of Address if possible.
    /// Returns None otherwise.
    pub fn as_id(&self) -> Option<u64> {
        if let Protocol::Id = self.protocol {
            let id = unsigned_varint::decode::u64(&self.payload)
                .expect("unsigned varint decode payload of ID Address shouldn't be fail; qed")
                .0;
            Some(id)
        } else {
            None
        }
    }

    /// Return the encoded bytes of address (protocol + payload).
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.payload.len());
        bytes.push(self.protocol as u8);
        bytes.extend_from_slice(self.payload());
        bytes
    }

    /// Return the checksum of (protocol + payload).
    pub fn checksum(&self) -> Vec<u8> {
        checksum(&self.as_bytes())
    }

    // A helper function for `from_str`.
    fn new_with_check(
        protocol: Protocol,
        raw: &[u8],
        payload_size: usize,
    ) -> Result<Self, AddressError> {
        let decoded = base32_decode(raw)?;
        let (payload, checksum) = decoded.split_at(decoded.len() - constant::CHECKSUM_HASH_LEN);
        if payload.len() != payload_size {
            return Err(AddressError::InvalidPayload);
        }

        let mut bytes = Vec::with_capacity(1 + payload_size);
        bytes.push(protocol as u8);
        bytes.extend_from_slice(payload);
        if !validate_checksum(&bytes, checksum) {
            return Err(AddressError::InvalidChecksum);
        }

        Ok(Self {
            protocol,
            payload: payload.to_vec(),
        })
    }
}

impl FromStr for Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 || s.len() > MAX_ADDRESS_STRING_LEN {
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
                if raw.len() > MAX_U64_LEN {
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
                PAYLOAD_HASH_LEN,
            ),
            Protocol::Actor => {
                Self::new_with_check(Protocol::Actor, raw.as_bytes(), PAYLOAD_HASH_LEN)
            }
            Protocol::Bls => {
                Self::new_with_check(Protocol::Bls, raw.as_bytes(), BLS_PUBLIC_KEY_LEN)
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

#[cfg(test)]
mod tests {
    use crate::{set_network, Address, Network};

    #[test]
    fn address_cbor_serde() {
        let id_addr = Address::new_id_addr(12_512_063u64).unwrap();
        let ser = minicbor::to_vec(&id_addr).unwrap();
        assert_eq!(ser, [69, 0, 191, 214, 251, 5]);
        let de = minicbor::decode::<Address>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }

    #[test]
    fn address_json_serde() {
        unsafe { set_network(Network::Test) };
        let id_addr = Address::new_id_addr(1024).unwrap();
        assert_eq!(id_addr.to_string(), "t01024");
        let ser = serde_json::to_string(&id_addr).unwrap();
        assert_eq!(ser, "\"t01024\"");
        let de = serde_json::from_str::<Address>(&ser).unwrap();
        assert_eq!(de, id_addr);
    }
}
