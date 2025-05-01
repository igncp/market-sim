use super::{Company, CompanyVerifyError};

impl Company {
    pub fn verify(&self) -> Result<(), CompanyVerifyError> {
        if self.name.is_empty() {
            return Err(CompanyVerifyError::Name);
        }
        self.symbol
            .verify()
            .map_err(|_| CompanyVerifyError::Symbol)?;

        Ok(())
    }
}
