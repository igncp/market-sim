use super::Simulation;
use crate::core::{
    company::{Companies, Ipos, ListedCompanies},
    investor::{Investor, Investors},
    market_maker::MarketMakers,
    money::{Currency, Money},
    price::Price,
    stock::{Stock, StockOwner},
    stock_exchange::StockExchange,
    time::TimeHandler,
};
use rand::{seq::SliceRandom, Rng};

impl Simulation {
    pub(super) fn create_valid_new_investor(
        &mut self,
        se: &StockExchange,
        time: &TimeHandler,
    ) -> Result<Investor, String> {
        let new_investor = 'investor_loop: loop {
            let potential = Investors::gen_list(1, time, &mut self.r)?;

            for investor in potential.mapping.values() {
                if !se.investors.mapping.contains_key(&investor.id) && investor.verify(time).is_ok()
                {
                    break 'investor_loop investor.clone();
                }
            }
        };

        return Ok(new_investor);
    }

    fn assign_stocks_to_investors(&mut self, se: &mut StockExchange) {
        let investors_list = se.investors.mapping.values().collect::<Vec<_>>();

        for company in &se.listed_companies.get_list() {
            let mut remaining_stocks = company.total_stocks;

            loop {
                let investor = investors_list.choose(&mut self.r).unwrap();
                let remaining_lots = remaining_stocks / company.lot_size;
                let random_lots = self.r.gen_range(1..=remaining_lots);
                let quantity = random_lots * company.lot_size;
                let value = Money::gen_from_range(&mut self.r, (1.0, 100.0));
                let price = Money {
                    value,
                    currency: Currency::Hkd,
                };
                let stock = Stock {
                    owner: StockOwner::Investor(investor.id),
                    price,
                    quantity,
                    symbol: company.symbol.clone(),
                };

                se.owned_stocks.entry_with_default(&stock.owner).push(stock);

                remaining_stocks -= quantity;

                if remaining_stocks == 0 {
                    break;
                }
            }
        }
    }

    fn calculate_prices(&mut self, se: &mut StockExchange) {
        for company in &se.listed_companies.get_list() {
            let company_stocks_prices = se.owned_stocks.get_prices(&company.symbol);

            let average = Money::calculate_average(&company_stocks_prices);
            let price = Price {
                ask: average,
                bid: average,
            };

            se.prices.0.insert(company.symbol.clone(), price);
        }
    }

    pub fn init(&mut self, se: &mut StockExchange, time: &TimeHandler) -> Result<(), String> {
        let companies = Companies::gen_list(&Default::default(), 100, &mut self.r)?;
        se.companies = companies;
        se.listed_companies = ListedCompanies::gen_list(&se.companies, &mut self.r)?;

        let ipos_companies = Companies::gen_list(&se.companies, 10, &mut self.r)?;
        se.companies.mapping.extend(ipos_companies.mapping.clone());
        se.ipos = Ipos::gen_list(&ipos_companies, time, &mut self.r)?;

        se.investors = Investors::gen_list(1000, time, &mut self.r)?;
        se.market_makers = MarketMakers::gen_list(10, time, &mut self.r)?;

        self.assign_stocks_to_investors(se);
        self.calculate_prices(se);

        // TODO:
        // - Create brokers (not all humans)

        Ok(())
    }
}
