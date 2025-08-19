mod matching_engine;

use matching_engine::engine::{MatchingEngine, TradingPair};

use crate::matching_engine::orderbook::{Order, OrderType};
fn main() {
    let mut matching_engine = MatchingEngine::new();
    let btc_trading_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    matching_engine.add_new_market(&btc_trading_pair);

    let buy_order = Order::new(OrderType::Bid, 2.3);

    let result = matching_engine.place_limit_order(btc_trading_pair, 10000.0, buy_order);
    match result {
        Ok(_) => println!("Order placed successfully"),
        Err(e) => println!("Error placing order: {}", e),
    }

    let eth_trading_pair = TradingPair::new("ETH".to_string(), "USD".to_string());

    matching_engine.add_new_market(&eth_trading_pair);

    let buy_order = Order::new(OrderType::Bid, 1.0);
    let result = matching_engine.place_limit_order(eth_trading_pair, 1000.0, buy_order);
    match result {
        Ok(_) => println!("Order placed successfully !!"),
        Err(e) => println!("Error placing order: {}", e),
    }
}
