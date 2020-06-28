use serde::{Deserialize, Serialize};
use super::utils::vec_cid_json;
use cid::Cid;
use super::header::{BlockHeader, ChainEpoch};

//pub struct TipsetKey {
//    #[serde(with = "vec_cid_json")]
//    cids: Vec<Cid>,
//}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tipset {
    #[serde(with = "vec_cid_json")]
    cids: Vec<Cid>,
    blocks: Vec<BlockHeader>,
    height: ChainEpoch,
}

