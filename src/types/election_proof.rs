use serde::{Deserialize, Serialize};

/// The PoSt election proof of space/time
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct ElectionProof {
    /// VRF proof
    #[serde(rename = "VRFProof")]
    #[serde(with = "bytes")]
    pub vrf_proof: Vec<u8>,
}