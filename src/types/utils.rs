
use num_bigint::BigInt;
use serde::{de, ser, Deserialize, Serialize};

/// JSON serialization
pub fn serialize<S>(int: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
{
    int.to_string().serialize(serializer)
}

/// JSON deserialization
pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
    where
        D: de::Deserializer<'de>,
{
    let bigint = String::deserialize(deserializer)?
        .parse::<BigInt>()
        .map_err(|e| de::Error::custom(e.to_string()))?;
    Ok(bigint)
}