use serde::{Deserialize, Serialize};
use cid::Cid;
use num_bigint::BigInt;

use super::address::Address;
use super::ticket::Ticket;
use super::proofs::{ElectionProof, PoStProof};
use super::crypto::Signature;
use super::utils::{vec_cid_json, cid_json, bytes_json, bigint_json};

pub type ChainEpoch = i64;

#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BeaconEntry {
    ///
    pub round: u64,
    ///
    #[serde(with = "bytes_json")]
    pub data: Vec<u8>,
}

/// The header part of the block.
#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockHeader {
    ///
    pub miner: Address,
    ///
    pub ticket: Ticket,
    ///
    pub election_proof: ElectionProof,
    ///
    pub beacon_entries: Vec<BeaconEntry>,
    ///
    #[serde(rename = "WinPoStProof")]
    pub win_post_proof: Vec<PoStProof>,
    ///
    #[serde(with = "vec_cid_json")]
    pub parents: Vec<Cid>,
    ///
    #[serde(with = "bigint_json")]
    pub parent_weight: BigInt,
    ///
    pub height: ChainEpoch,
    ///
    #[serde(with = "cid_json")]
    pub parent_state_root: Cid,
    ///
    #[serde(with = "cid_json")]
    pub parent_message_receipts: Cid,
    ///
    #[serde(with = "cid_json")]
    pub messages: Cid,
    ///
    #[serde(rename = "BLSAggregate")]
    pub bls_aggregate: Signature,
    ///
    pub timestamp: u64,
    ///
    pub block_sig: Signature,
    ///
    pub fork_signaling: u64,
    /*
    /// internal
    #[serde(skip)]
    validated: bool, // true if the signature has been validated
    */
}