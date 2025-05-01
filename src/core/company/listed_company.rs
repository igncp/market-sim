use super::{ListedCompanies, ListedCompany, ListedCompanyVerifyError};

impl ListedCompany {
    pub fn verify(&self) -> Result<(), ListedCompanyVerifyError> {
        if self.lot_size == 0 {
            return Err(ListedCompanyVerifyError::LotSize);
        }
        if self.total_stocks == 0 || self.total_stocks % self.lot_size != 0 {
            return Err(ListedCompanyVerifyError::TotalStocks);
        }
        self.symbol
            .verify()
            .map_err(|_| ListedCompanyVerifyError::Symbol)?;

        Ok(())
    }
}

impl ListedCompanies {
    pub fn get_list(&self) -> Vec<ListedCompany> {
        self.mapping.values().cloned().collect::<Vec<_>>()
    }
}
