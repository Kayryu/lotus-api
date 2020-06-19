use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Ticket {
    /// VRF proof
    #[serde(rename = "VRFProof")]
    #[serde(with = "bytes")]
    pub vrf_proof: Vec<u8>,
}