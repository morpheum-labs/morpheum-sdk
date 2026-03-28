//! KlineClient — queries for mark price, VWAP, sentiment, last kline, and kline snapshots.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::kline::v1 as proto;

use crate::requests;
use crate::types::{KlineData, LastKline, LongShortRatio, MarkPriceWithSpread, Vwap};

/// Primary client for kline module queries.
pub struct KlineClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl KlineClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets mark price with spread bands for a market at a logical timestamp.
    pub async fn get_mark_price_with_spread(
        &self, market_index: u64, logical_timestamp: u64,
    ) -> Result<MarkPriceWithSpread, SdkError> {
        let req = requests::GetMarkPriceWithSpreadRequest::new(market_index, logical_timestamp);
        let proto_req: proto::GetMarkPriceWithSpreadRequest = req.into();
        let resp = self.query("/kline.v1.Query/GetMarkPriceWithSpread", proto_req.encode_to_vec()).await?;
        let p = proto::GetMarkPriceWithSpreadResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(MarkPriceWithSpread {
            mark: p.mark, spread: p.spread,
            lower_band: p.lower_band, upper_band: p.upper_band,
            quality: p.quality, proof: p.proof,
        })
    }

    /// Gets VWAP over a logical time range for a market.
    pub async fn get_vwap(
        &self, market_index: u64, start_logical: u64, end_logical: u64,
    ) -> Result<Vwap, SdkError> {
        let req = requests::GetVwapRequest::new(market_index, start_logical, end_logical);
        let proto_req: proto::GetVwapRequest = req.into();
        let resp = self.query("/kline.v1.Query/GetVWAP", proto_req.encode_to_vec()).await?;
        let p = proto::GetVwapResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(Vwap { vwap: p.vwap, total_volume: p.total_volume, proof: p.proof })
    }

    /// Gets long/short ratio for a market, period, and candle open time.
    pub async fn get_long_short_ratio(
        &self, market_index: u64, period: u32, open_at_logical: u64,
    ) -> Result<LongShortRatio, SdkError> {
        let req = requests::GetLongShortRatioRequest::new(market_index, period, open_at_logical);
        let proto_req: proto::GetLongShortRatioRequest = req.into();
        let resp = self.query("/kline.v1.Query/GetLongShortRatio", proto_req.encode_to_vec()).await?;
        let p = proto::GetLongShortRatioResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(LongShortRatio {
            ratio: p.ratio, long_oi: p.long_oi, short_oi: p.short_oi, proof: p.proof,
        })
    }

    /// Gets the last completed OHLC kline for a market/period.
    pub async fn get_last_kline(
        &self, market_index: u64, period: u32, logical_timestamp: u64,
    ) -> Result<LastKline, SdkError> {
        let req = requests::GetLastKlineRequest::new(market_index, period, logical_timestamp);
        let proto_req: proto::GetLastKlineRequest = req.into();
        let resp = self.query("/kline.v1.Query/GetLastKline", proto_req.encode_to_vec()).await?;
        let p = proto::GetLastKlineResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(LastKline {
            open: p.open, high: p.high, low: p.low, close: p.close,
            volume_base: p.volume_base, proof: p.proof,
        })
    }

    /// Gets a batch of klines over a time range for a market/interval.
    pub async fn query_klines_snapshot(
        &self, market_index: u64, interval: &str, start_time: u64, end_time: u64,
    ) -> Result<Vec<KlineData>, SdkError> {
        let req = requests::QueryKlinesSnapshotRequest::new(market_index, interval, start_time, end_time);
        let proto_req: proto::QueryKlinesSnapshotRequest = req.into();
        let resp = self.query("/kline.v1.Query/QueryKlinesSnapshot", proto_req.encode_to_vec()).await?;
        let p = proto::QueryKlinesSnapshotResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.klines.into_iter().map(Into::into).collect())
    }
}

#[async_trait(?Send)]
impl MorpheumClient for KlineClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/kline.v1.Query/GetMarkPriceWithSpread" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetMarkPriceWithSpreadResponse {
                        mark: 50_000_000_000, spread: 100_000_000,
                        lower_band: 49_900_000_000, upper_band: 50_100_000_000,
                        quality: 95_000_000, proof: vec![0u8; 32],
                    }))
                }
                "/kline.v1.Query/GetVWAP" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetVwapResponse {
                        vwap: 50_050_000_000, total_volume: "1000000".into(), proof: vec![],
                    }))
                }
                "/kline.v1.Query/GetLongShortRatio" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetLongShortRatioResponse {
                        ratio: 150_000_000, long_oi: "600".into(), short_oi: "400".into(), proof: vec![],
                    }))
                }
                "/kline.v1.Query/GetLastKline" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetLastKlineResponse {
                        open: 49_500, high: 51_000, low: 49_000, close: 50_500,
                        volume_base: "500".into(), proof: vec![],
                    }))
                }
                "/kline.v1.Query/QueryKlinesSnapshot" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryKlinesSnapshotResponse {
                        success: true, klines: vec![],
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> KlineClient {
        KlineClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_mark_price_with_spread_works() {
        let mp = make_client().get_mark_price_with_spread(1, 100).await.unwrap();
        assert_eq!(mp.mark, 50_000_000_000);
        assert_eq!(mp.quality, 95_000_000);
    }

    #[tokio::test]
    async fn get_vwap_works() {
        let v = make_client().get_vwap(1, 100, 200).await.unwrap();
        assert_eq!(v.vwap, 50_050_000_000);
        assert_eq!(v.total_volume, "1000000");
    }

    #[tokio::test]
    async fn get_long_short_ratio_works() {
        let r = make_client().get_long_short_ratio(1, 4, 100).await.unwrap();
        assert_eq!(r.ratio, 150_000_000);
        assert_eq!(r.long_oi, "600");
    }

    #[tokio::test]
    async fn get_last_kline_works() {
        let k = make_client().get_last_kline(1, 4, 100).await.unwrap();
        assert_eq!(k.close, 50_500);
    }

    #[tokio::test]
    async fn query_klines_snapshot_works() {
        let klines = make_client().query_klines_snapshot(1, "1h", 100, 200).await.unwrap();
        assert!(klines.is_empty());
    }
}
