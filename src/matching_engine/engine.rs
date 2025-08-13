use super::orderbook::{Order, OrderBook};
use std::{collections::HashMap, thread::LocalKey};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, trading_pair: TradingPair) {
        self.orderbooks
            .insert(trading_pair.clone(), OrderBook::new());
        println!("New market added: {}", trading_pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        trading_pair: TradingPair,
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        let orderbook = self.orderbooks.get_mut(&trading_pair);

        match orderbook {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                Ok(())
            }
            None => Err(format!("Market not found: {}", trading_pair.to_string())),
        }
    }
}
