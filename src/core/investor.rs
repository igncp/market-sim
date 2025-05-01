use std::collections::HashMap;

use super::{
    money::{Money, MoneyVerifyError},
    time::TimeHandler,
};
use rand::{rngs::StdRng, seq::SliceRandom};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Copy)]
#[serde(transparent)]
pub struct InvestorId(u64);

impl InvestorId {
    pub fn new(previous: &Self) -> Self {
        Self(previous.0 + 1)
    }

    pub fn init() -> Self {
        Self(0)
    }
}

impl Default for InvestorId {
    fn default() -> Self {
        Self::init()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Investor {
    pub debt: Money,
    pub dob: u64, // UNIX timestamp
    pub id: InvestorId,
    pub liquid_cash: Money,
    pub name: String,
}

#[derive(Debug)]
pub enum InvestorVerifyError {
    InvalidName,
    InvalidAge,
    MoneyError(MoneyVerifyError),
}

impl Investor {
    pub fn verify(&self, time: &TimeHandler) -> Result<(), InvestorVerifyError> {
        self.liquid_cash
            .verify()
            .map_err(InvestorVerifyError::MoneyError)?;

        if self.name.is_empty() {
            return Err(InvestorVerifyError::InvalidName);
        }

        if self.get_age(time) < 18.0 || self.get_age(time) > 100.0 {
            Err(InvestorVerifyError::InvalidAge)
        } else {
            Ok(())
        }
    }

    pub fn get_age(&self, time: &TimeHandler) -> f64 {
        let now = time.get_now_unix_timestamp();

        (now - self.dob) as f64 / 60.0 / 60.0 / 24.0 / 365.25
    }
}

impl Investor {
    pub fn subtract_cash(&mut self, amount: &Money) {
        let diff = self.liquid_cash.value - amount.value;

        if diff < Decimal::ZERO {
            self.liquid_cash.value = Decimal::ZERO;
            self.debt.value += diff.abs();
        } else {
            self.liquid_cash.value = diff;
        }
    }

    pub fn add_cash(&mut self, amount: &Money) {
        if self.debt.value > Decimal::ZERO {
            let diff = self.debt.value - amount.value;

            if diff < Decimal::ZERO {
                self.debt.value = Decimal::ZERO;
                self.liquid_cash.value += diff.abs();
            } else {
                self.debt.value = diff;
            }
        } else {
            self.liquid_cash.value += amount.value;
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Investors {
    pub last_id: InvestorId,
    pub mapping: HashMap<InvestorId, Investor>,
}

impl Investors {
    pub fn get_random(&self, r: &mut StdRng) -> &Investor {
        let investors_list = self.mapping.values().collect::<Vec<_>>();

        investors_list.choose(r).unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_investor_id() {
        use super::InvestorId;

        #[derive(Debug, PartialEq, Eq, Hash)]
        struct Foo(u64);

        let foo = Foo(0);

        assert!(foo == Foo(0));

        let id = InvestorId::init();
        assert_eq!(id.0, 0);

        // Type error
        // assert_eq!(id, foo);

        let id = InvestorId::new(&id);
        assert_eq!(id.0, 1);
    }
}
