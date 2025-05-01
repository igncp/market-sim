use std::collections::HashMap;

use super::money::Money;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct BrokerId(u64);

impl BrokerId {
    pub fn new(previous: &Self) -> Self {
        Self(previous.0 + 1)
    }

    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BrokerType {
    Human,
    Web,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Broker {
    pub broker_type: BrokerType,
    pub handling_fee: Money,
    pub id: BrokerId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Brokers {
    pub last_id: BrokerId,
    pub mapping: HashMap<BrokerId, Broker>,
}

impl Default for Brokers {
    fn default() -> Self {
        Brokers {
            last_id: BrokerId::init(),
            mapping: HashMap::new(),
        }
    }
}
