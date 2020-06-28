use super::JsonApi;
use crate::error::Result;
use crate::helper;
use cid::Cid;
use crate::types::{Bytes, Tipset, DomainSeparationTag, ChainEpoch, Randomness, BytesRef};
use crate::types::utils::vec_cid_json;

#[async_trait::async_trait]
pub trait ChainApi: JsonApi {
    async fn chain_head(&self) -> Result<Tipset> {
        self.request("ChainHead", vec![]).await
    }

//    async fn chain_get_randomness(
//        &self,
//        key: &Vec<Cid>,
//        personalization: &DomainSeparationTag,
//        rand_epoch: ChainEpoch,
//        entropy: &[u8],
//    ) -> Result<Randomness> {
//        self.request(
//            "ChainGetRandomness",
//            vec![
//                helper::serialize(key),
//                helper::serialize(personalization),
//                helper::serialize(&rand_epoch),
//                helper::serialize(&BytesRef::from(entropy)),
//            ],
//        )
//            .await
//    }
//
//    async fn chain_get_block(&self, cid: &Cid) -> Result<BlockHeader> {
//        self.request("ChainGetBlock", vec![helper::serialize(cid)])
//            .await
//    }
//
//    async fn chain_get_tipset(&self, key: &TipsetKey) -> Result<Tipset> {
//        self.request("ChainGetTipSet", vec![helper::serialize(key)])
//            .await
//    }
//
//    async fn chain_get_block_messages(&self, cid: &Cid) -> Result<BlockMessages> {
//        self.request("ChainGetBlockMessages", vec![helper::serialize(cid)])
//            .await
//    }
//
//    async fn chain_get_parent_receipts(&self, cid: &Cid) -> Result<Vec<MessageReceipt>> {
//        self.request("ChainGetParentReceipts", vec![helper::serialize(cid)])
//            .await
//    }
//
//    async fn chain_get_parent_messages(&self, cid: &Cid) -> Result<Vec<ParentMessage>> {
//        self.request("ChainGetParentMessages", vec![helper::serialize(cid)])
//            .await
//    }
//
//    async fn chain_get_tipset_by_height(
//        &self,
//        height: ChainEpoch,
//        key: &TipsetKey,
//    ) -> Result<Tipset> {
//        self.request(
//            "ChainGetTipSetByHeight",
//            vec![helper::serialize(&height), helper::serialize(key)],
//        )
//            .await
//    }
//
//    async fn chain_read_obj(&self, cid: &Cid) -> Result<Vec<u8>> {
//        let bytes: Bytes = self
//            .request("ChainReadObj", vec![helper::serialize(cid)])
//            .await?;
//        Ok(bytes.into_inner())
//    }
//
//    async fn chain_has_obj(&self, cid: &Cid) -> Result<bool> {
//        self.request("ChainHasObj", vec![helper::serialize(cid)])
//            .await
//    }
//
//    async fn chain_stat_obj(&self, obj: &Cid, base: &Cid) -> Result<ObjStat> {
//        self.request(
//            "ChainHasObj",
//            vec![helper::serialize(obj), helper::serialize(base)],
//        )
//            .await
//    }
//
//    async fn chain_set_head(&self, key: &TipsetKey) -> Result<()> {
//        self.request("ChainSetHead", vec![helper::serialize(key)])
//            .await
//    }
//
//    async fn chain_get_genesis(&self) -> Result<Tipset> {
//        self.request("ChainGetGenesis", vec![]).await
//    }
//
//    async fn chain_tipset_weight(&self, key: &TipsetKey) -> Result<BigInt> {
//        let bigint: BigIntWrapper = self
//            .request("ChainTipSetWeight", vec![helper::serialize(key)])
//            .await?;
//        Ok(bigint.into_inner())
//    }
//
//    async fn chain_get_message(&self, cid: &Cid) -> Result<UnsignedMessage> {
//        self.request("ChainGetMessage", vec![helper::serialize(cid)])
//            .await
//    }
//
//    async fn chain_get_path(&self, from: &TipsetKey, to: &TipsetKey) -> Result<Vec<HeadChange>> {
//        self.request(
//            "ChainGetPath",
//            vec![helper::serialize(from), helper::serialize(to)],
//        )
//            .await
//    }
//
//    async fn chain_export(
//        &self,
//        key: &TipsetKey,
//    ) -> Result<(SubscriptionId, NotificationStream<Bytes>)> {
//        self.subscribe("ChainExport", vec![helper::serialize(key)])
//            .await
//    }
}