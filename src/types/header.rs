use serde::{Deserialize, Serialize};

use super::address::Address;
use super::ticket::Ticket;
use super::election_proof::ElectionProof;

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
    pub parents: Vec<Cid>,
    ///
    #[serde(with = "bigint_json")]
    pub parent_weight: BigInt,
    ///
    pub height: ChainEpoch,
    ///
    pub parent_state_root: Cid,
    ///
    pub parent_message_receipts: Cid,
    ///
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