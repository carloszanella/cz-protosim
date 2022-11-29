use ethereum_types::{Address, U256};
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct ERC20Token {
    pub chain: String,
    pub decimals: u64,
    pub symbol: String,
    pub address: Address,
}

pub struct PoolSimulationError {}

pub trait Pool {
    fn spot_price(&self, a: &ERC20Token, b: &ERC20Token) -> Decimal;
    fn fee(&self, a: &ERC20Token, b: &ERC20Token) -> f64;
    fn get_amount_out(
        &self,
        sell_amount: U256,
        sell_token: &ERC20Token,
        buy_token: &ERC20Token,
    ) -> (U256, Self);
    fn inertia(&self, a: &ERC20Token, b: &ERC20Token) -> U256;
}
