use std::collections::HashMap;

use ethereum_types::{Address, U256, H160};
use models::ERC20Token;
use uniswap_v2::UniswapV2Pool;

pub mod models;
pub mod uniswap_v2;

fn main() {
    let token_0 = ERC20Token {
        chain: String::from("ethereum"),
        decimals: 18,
        symbol: String::from("ShitC"),
        address: Address::zero(),
    };
    let token_1 = ERC20Token {
        chain: String::from("ethereum"),
        decimals: 18,
        symbol: String::from("ShitC"),
        address: Address::zero(),
    };
    let mut reserves = HashMap::new();
    reserves.insert(token_0.symbol.clone(), U256::from(10000000000_u64));
    reserves.insert(token_1.symbol.clone(), U256::from(10000000000_u64));

    let _usv2pool = UniswapV2Pool {
        address: H160::zero(),
        token_0: token_0,
        token_1: token_1,
        reserves: reserves,
    };
}
