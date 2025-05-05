use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod base;
mod company_symbol;
mod ipo;
mod listed_company;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Ord)]
#[serde(transparent)]
pub struct CompanySymbol(pub(crate) String);

impl PartialOrd for CompanySymbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub enum CompanySymbolVerifyError {
    Symbol,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub symbol: CompanySymbol,
}

pub enum CompanyVerifyError {
    Name,
    Symbol,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Companies {
    pub mapping: BTreeMap<CompanySymbol, Company>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListedCompany {
    pub lot_size: u64,
    pub symbol: CompanySymbol,
    pub total_stocks: u64,
}

#[derive(Debug)]
pub enum ListedCompanyVerifyError {
    LotSize,
    Name,
    Symbol,
    TotalStocks,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ListedCompanies {
    pub mapping: BTreeMap<CompanySymbol, ListedCompany>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ipo {
    pub symbol: CompanySymbol,
    pub shares: u64,
    pub lot_size: u64,
    pub date: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Ipos {
    pub mapping: BTreeMap<CompanySymbol, Ipo>,
}
