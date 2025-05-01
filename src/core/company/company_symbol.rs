use super::{CompanySymbol, CompanySymbolVerifyError};

impl CompanySymbol {
    pub fn new(symbol: String) -> Self {
        CompanySymbol(symbol)
    }

    pub fn verify(&self) -> Result<(), CompanySymbolVerifyError> {
        if self.0.is_empty() || self.0.contains(|c: char| !c.is_alphabetic()) {
            return Err(CompanySymbolVerifyError::Symbol);
        }

        Ok(())
    }

    pub fn cmp(&self, other: &CompanySymbol) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
