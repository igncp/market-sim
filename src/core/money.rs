use rand::{rngs::StdRng, Rng};
use rust_decimal::{prelude::FromPrimitive, prelude::ToPrimitive, Decimal};
use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Default)]
pub enum Currency {
    #[default]
    Hkd,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Money {
    pub currency: Currency,
    pub value: Decimal,
}

#[derive(Debug)]
pub enum MoneyVerifyError {
    NegativeValue,
    TooManyDecimals,
}

impl Money {
    pub fn verify(&self) -> Result<(), MoneyVerifyError> {
        let cash_decimals_digits = self.value.to_string().split('.').nth(1).unwrap_or("").len();

        if self.value.lt(&Decimal::new(0, 0)) {
            return Err(MoneyVerifyError::NegativeValue);
        }

        if cash_decimals_digits > 2 {
            Err(MoneyVerifyError::TooManyDecimals)
        } else {
            Ok(())
        }
    }

    pub fn calculate_average(prices: &[Money]) -> Money {
        let total: Decimal = prices.iter().map(|price| price.value).sum();
        let average = total
            .checked_div(Decimal::new(prices.len() as i64, 0))
            .unwrap();
        let value = average.round_dp(2);

        Money {
            currency: prices[0].currency,
            value,
        }
    }

    pub fn gen_from_range(r: &mut StdRng, range: (f64, f64)) -> Decimal {
        Decimal::from_f64(r.gen_range(range.0..=range.1))
            .unwrap()
            .round_dp(2)
    }

    pub fn from_u64(value: u64) -> Decimal {
        Decimal::from_u64(value).unwrap().round_dp(2)
    }

    pub fn from_f64(value: f64) -> Decimal {
        Decimal::from_f64(value).unwrap().round_dp(2)
    }

    pub fn to_f64(&self) -> f64 {
        self.value.to_f64().unwrap()
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        if self.currency != other.currency {
            panic!("Cannot add money of different currencies");
        }

        Money {
            currency: self.currency,
            value: self.value + other.value,
        }
    }
}
