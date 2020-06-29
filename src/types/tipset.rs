use serde::{ser, de, Deserialize, Serialize};
use super::utils::vec_cid_json;
use cid::Cid;
use super::header::{BlockHeader, ChainEpoch};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TipSet {
    #[serde(with = "vec_cid_json")]
    cids: Vec<Cid>,
    blocks: Vec<BlockHeader>,
    height: ChainEpoch,
}

#[derive(Clone, Debug)]
pub struct TipSetKey {
    cids: Vec<Cid>,
}

// Implement JSON serialization for TipsetKey.
impl ser::Serialize for TipSetKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        vec_cid_json::serialize(&self.cids, serializer)
    }
}

// Implement JSON deserialization for TipsetKey.
impl<'de> de::Deserialize<'de> for TipSetKey {
    fn deserialize<D>(deserializer: D) -> Result<TipSetKey, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        let cids = vec_cid_json::deserialize(deserializer)?;
        Ok(TipSetKey { cids })
    }
}
