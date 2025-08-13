mod matching_engine;

use matching_engine::engine::{MatchingEngine, TradingPair};
fn main() {
    let mut matching_engine = MatchingEngine::new();
    let trading_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    matching_engine.add_new_market(trading_pair);
}
