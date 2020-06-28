use serde::{Deserialize, Serialize};
use super::utils::bytes_json;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Ticket {
    /// VRF proof
    #[serde(rename = "VRFProof")]
    #[serde(with = "bytes_json")]
    pub vrf_proof: Vec<u8>,
}