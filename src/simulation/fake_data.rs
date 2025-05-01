use crate::core::{
    company::{Companies, Company, CompanySymbol, Ipo, Ipos, ListedCompanies, ListedCompany},
    investor::{Investor, InvestorId, Investors},
    market_maker::{MarketMaker, MarketMakerId, MarketMakers},
    money::{Currency, Money},
    time::TimeHandler,
};
use fake::{
    faker::{company, name},
    Fake,
};
use rand::{rngs::StdRng, Rng};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::collections::HashSet;

impl Companies {
    pub fn gen_list(existing: &Self, n: usize, rng: &mut StdRng) -> Result<Self, String> {
        let mut symbols: HashSet<CompanySymbol> = HashSet::default();
        let mut list = Vec::with_capacity(n);
        let mut failures = 0;

        for (symbol, _) in existing.mapping.iter() {
            symbols.insert(symbol.clone());
        }

        let allowed_failures = 10 * n;

        loop {
            let (symbol, name) = loop {
                let tmp_name: String = match rng.gen::<u8>() % 5 {
                    0 => company::en::CompanyName().fake(),
                    1 => company::zh_tw::CompanyName().fake(),
                    2 => company::pt_br::CompanyName().fake(),
                    3 => company::ja_jp::CompanyName().fake(),
                    _ => company::fr_fr::CompanyName().fake(),
                };

                let tmp_symbol_str: String = tmp_name
                    .to_uppercase()
                    .replace(" ", "")
                    .chars()
                    .take(4)
                    .collect();
                let tmp_symbol = CompanySymbol::new(tmp_symbol_str);

                if !symbols.contains(&tmp_symbol) {
                    symbols.insert(tmp_symbol.clone());
                    break (tmp_symbol, tmp_name);
                }
            };

            let company = Company {
                name,
                symbol: symbol.clone(),
            };

            if company.verify().is_ok() {
                list.push(company);
            } else {
                failures += 1;
            }

            if failures > allowed_failures {
                return Err("Failed to generate companies".to_string());
            }

            if list.len() == n {
                break;
            }
        }

        Ok(Companies {
            mapping: list.iter().map(|c| (c.symbol.clone(), c.clone())).collect(),
        })
    }
}

impl ListedCompanies {
    pub fn gen_list(companies: &Companies, rng: &mut StdRng) -> Result<Self, String> {
        let mut list = Vec::with_capacity(companies.mapping.len());

        for (_, company) in companies.mapping.iter() {
            let lot_size = (100.0 / (rng.gen::<f64>() + 1.0)).ceil() as u64;
            let total_stocks = rng.gen_range(10..100) * lot_size;

            let company = ListedCompany {
                lot_size,
                total_stocks,
                symbol: company.symbol.clone(),
            };

            if company.verify().is_err() {
                return Err("Failed to generate listed companies".to_string());
            }

            list.push(company);
        }

        let mapping =
            list.into_iter()
                .fold(std::collections::HashMap::new(), |mut acc, company| {
                    acc.insert(company.symbol.clone(), company);
                    acc
                });

        Ok(ListedCompanies { mapping })
    }
}

impl Ipos {
    pub fn gen_list(
        companies: &Companies,
        time: &TimeHandler,
        rng: &mut StdRng,
    ) -> Result<Self, String> {
        let mut list = Vec::with_capacity(companies.mapping.len());

        for (_, company) in companies.mapping.iter() {
            let lot_size = (100.0 / (rng.gen::<f64>() + 1.0)).ceil() as u64;
            let total_stocks = rng.gen_range(10..100) * lot_size;
            let random_days = rng.gen_range(1..30);

            let company = Ipo {
                date: time.get_n_days_from_now_unix_timestamp(random_days),
                lot_size,
                shares: total_stocks,
                symbol: company.symbol.clone(),
            };

            list.push(company);
        }

        let mapping =
            list.into_iter()
                .fold(std::collections::HashMap::new(), |mut acc, company| {
                    acc.insert(company.symbol.clone(), company);
                    acc
                });

        Ok(Ipos { mapping })
    }
}

impl Investors {
    pub fn gen_list(n: usize, time: &TimeHandler, rng: &mut StdRng) -> Result<Self, String> {
        let mut names: HashSet<String> = HashSet::default();
        let mut list = Vec::with_capacity(n);
        let mut failures = 0;

        let allowed_failures = 10 * n;
        let mut last_investor_id = InvestorId::init();

        loop {
            let name = loop {
                let tmp_name: String = match rng.gen_range(0..5) % 5 {
                    0 => name::en::Name().fake_with_rng(rng),
                    1 => name::zh_tw::Name().fake_with_rng(rng),
                    2 => name::pt_br::Name().fake_with_rng(rng),
                    3 => name::ja_jp::Name().fake_with_rng(rng),
                    _ => name::fr_fr::Name().fake_with_rng(rng),
                };

                if !names.contains(&tmp_name) {
                    names.insert(tmp_name.clone());
                    break tmp_name;
                }
            };

            let cash_raw = Decimal::from_f64(rng.gen::<f64>() * 100_000.0)
                .unwrap()
                .round_dp(2);
            let liquid_cash = Money {
                value: cash_raw,
                currency: Currency::Hkd,
            };
            let dob = rng.gen::<u64>() % 1_000_000_000;
            let debt = Money {
                currency: Currency::Hkd,
                value: Decimal::from_f64(0.0).unwrap().round_dp(2),
            };
            last_investor_id = InvestorId::new(&last_investor_id);
            let investor = Investor {
                debt,
                dob,
                id: last_investor_id,
                liquid_cash,
                name,
            };

            if investor.verify(time).is_ok() {
                list.push(investor);
            } else {
                failures += 1;
            }

            if failures > allowed_failures {
                return Err("Failed to generate investors".to_string());
            }

            if list.len() == n {
                break;
            }
        }

        let investors = Self {
            last_id: last_investor_id,
            mapping: list.iter().map(|i| (i.id, i.clone())).collect(),
        };

        Ok(investors)
    }
}

impl MarketMakers {
    pub fn gen_list(n: usize, time: &TimeHandler, rng: &mut StdRng) -> Result<Self, String> {
        let mut list = Vec::with_capacity(n);
        let mut last_id = MarketMakerId::init();

        loop {
            last_id = MarketMakerId::new(&last_id);
            let permit_time = rng.gen_range(1_000..1_000_000);
            let mm = MarketMaker {
                id: last_id,
                permit_start_time: time.get_now_unix_timestamp(),
                permit_end_time: time.get_now_unix_timestamp() + permit_time,
            };

            if mm.verify(time).is_ok() {
                list.push(mm);
            }

            if list.len() == n {
                break;
            }
        }

        let mms = Self {
            last_id,
            mapping: list.iter().map(|i| (i.id, i.clone())).collect(),
        };

        Ok(mms)
    }
}
