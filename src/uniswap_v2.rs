use std::collections::HashMap;

use ethereum_types::{H160, U256};
use rust_decimal::Decimal;

use crate::models::{ERC20Token, Pool};

#[derive(Clone)]
pub struct UniswapV2Pool {
    pub address: H160,
    pub token_0: ERC20Token,
    pub token_1: ERC20Token,
    pub reserves: HashMap<String, U256>,
}

impl UniswapV2Pool {
    const FEE: u64 = 30000;
    const FEE_PRECISION: u64 = 10000000;
}

impl Pool for UniswapV2Pool {
    fn spot_price(&self, a: &ERC20Token, b: &ERC20Token) -> Decimal {
        let r1 = self.reserves.get(&a.symbol).unwrap();
        let r2 = self.reserves.get(&b.symbol).unwrap();

        return Decimal::new(
            (r1 * 10_u64.pow(b.decimals as u32) / r2).as_u64() as i64,
            b.decimals as u32,
        );
    }
    fn fee(&self, a: &ERC20Token, b: &ERC20Token) -> f64 {
        return Self::FEE as f64 / Self::FEE_PRECISION as f64;
    }
    fn get_amount_out(
        &self,
        sell_amount: U256,
        sell_token: &ERC20Token,
        buy_token: &ERC20Token,
    ) -> (U256, Self) {
        let reserves_sell = self.reserves.get(&sell_token.symbol).unwrap();
        let reserves_buy = self.reserves.get(&buy_token.symbol).unwrap();
        let sell_amount_less_fee: U256 = sell_amount * (Self::FEE_PRECISION - Self::FEE);
        let numerator = sell_amount_less_fee * reserves_buy;
        let denominator: U256 = (reserves_sell * Self::FEE_PRECISION) + sell_amount_less_fee;

        let amount_out = numerator / denominator;

        let new_sell_reserves = reserves_sell + sell_amount;
        let new_buy_reserves = reserves_buy - amount_out;

        let mut reserves = HashMap::new();
        reserves.insert(buy_token.symbol.clone(), new_buy_reserves);
        reserves.insert(sell_token.symbol.clone(), new_sell_reserves);

        let mut updated_pool = self.clone();
        updated_pool.reserves = reserves;

        return (amount_out, updated_pool);
    }

    fn inertia(&self, a: &ERC20Token, b: &ERC20Token) -> U256 {
        return U256::zero();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ethereum_types::{Address, H160, U256};
    use rust_decimal::Decimal;

    use crate::models::{ERC20Token, Pool};

    use super::UniswapV2Pool;

    fn create_test_pool() -> UniswapV2Pool {
        let token_0 = ERC20Token {
            chain: String::from("ethereum"),
            decimals: 18,
            symbol: String::from("ShitCoin1"),
            address: Address::zero(),
        };
        let token_1 = ERC20Token {
            chain: String::from("ethereum"),
            decimals: 18,
            symbol: String::from("ShitCoin2"),
            address: Address::zero(),
        };

        let mut reserves = HashMap::new();
        reserves.insert(token_0.symbol.clone(), U256::from(15000000000_u64));
        reserves.insert(token_1.symbol.clone(), U256::from(10000000000_u64));

        let usv2pool = UniswapV2Pool {
            address: H160::zero(),
            token_0: token_0,
            token_1: token_1,
            reserves: reserves,
        };

        return usv2pool;
    }

    #[test]
    fn test_pool_attributes() {
        let usv2_pool = create_test_pool();
        let (a, b) = (&usv2_pool.token_0, &usv2_pool.token_1);

        assert_eq!(
            usv2_pool.reserves.get(&a.symbol).unwrap(),
            &U256::from_str_radix("15000000000", 10).unwrap()
        );
        assert_eq!(usv2_pool.fee(a, b), 0.003)
    }

    #[test]
    fn test_spot_prices() {
        let usv2_pool = create_test_pool();
        let (a, b) = (&usv2_pool.token_0, &usv2_pool.token_1);

        assert_eq!(
            usv2_pool.spot_price(&a, &b),
            Decimal::from_str_exact("1.500000000000000000").unwrap()
        );
        assert_eq!(
            usv2_pool.spot_price(&b, &a),
            Decimal::from_str_exact("0.666666666666666666").unwrap()
        );
    }

    #[test]
    fn test_get_amount_out() {
        let usv2_pool = create_test_pool();
        let (a, b) = (&usv2_pool.token_0, &usv2_pool.token_1);

        let (amount, new_pool) = usv2_pool.get_amount_out(U256::from(100000_u64), a, b);

        assert!(amount.gt(&U256::zero()));
        assert_eq!(new_pool.address, usv2_pool.address);
        assert_ne!(new_pool.reserves, usv2_pool.reserves);
    }

    #[test]
    fn test_real_pool() {
        let token_0 = ERC20Token {
            chain: String::from("ethereum"),
            decimals: 18,
            symbol: String::from("ShitCoin1"),
            address: Address::zero(),
        };
        let token_1 = ERC20Token {
            chain: String::from("ethereum"),
            decimals: 18,
            symbol: String::from("ShitCoin2"),
            address: Address::zero(),
        };

        let mut reserves = HashMap::new();
        reserves.insert(
            token_0.symbol.clone(),
            U256::from_str_radix("6770398782322527849696614", 10).unwrap(),
        );
        reserves.insert(
            token_1.symbol.clone(),
            U256::from_str_radix("5124813135806900540214", 10).unwrap(),
        );

        let usv2pool = UniswapV2Pool {
            address: H160::zero(),
            token_0: token_0,
            token_1: token_1,
            reserves: reserves,
        };
        let (amount_out, _) = usv2pool.get_amount_out(
            U256::from_str_radix("10000000000000000000000", 10).unwrap(),
            &usv2pool.token_0,
            &usv2pool.token_1,
        );

        assert_eq!(
            amount_out,
            U256::from_str_radix("7535635391574243447", 10).unwrap()
        )
    }
}
