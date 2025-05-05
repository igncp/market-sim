use std::collections::BTreeMap;

use super::time::TimeHandler;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord,
)]
#[serde(transparent)]
pub struct MarketMakerId(u64);

impl MarketMakerId {
    pub fn new(previous: &Self) -> Self {
        Self(previous.0 + 1)
    }

    pub fn init() -> Self {
        Self(0)
    }
}

// Market makers are considered to have unlimited liquidity
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketMaker {
    pub id: MarketMakerId,
    pub permit_end_time: u64,
    pub permit_start_time: u64,
}

pub enum MarketMakerVerifyError {
    InvalidTime,
}

impl MarketMaker {
    pub fn verify(&self, time: &TimeHandler) -> Result<(), MarketMakerVerifyError> {
        if self.permit_start_time >= self.permit_end_time {
            return Err(MarketMakerVerifyError::InvalidTime);
        }

        if self.permit_start_time < time.get_now_unix_timestamp() {
            return Err(MarketMakerVerifyError::InvalidTime);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct MarketMakers {
    pub last_id: MarketMakerId,
    pub mapping: BTreeMap<MarketMakerId, MarketMaker>,
}
