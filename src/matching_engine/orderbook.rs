use std::collections::HashMap;

#[derive(Debug)]
pub enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

#[derive(Debug)]
pub struct FillResult {
    pub filled_size: f64,
    pub remaining_size: f64,
    pub avg_fill_price: f64,
    pub filled_orders: Vec<FilledOrder>,
}

#[derive(Debug)]
pub struct FilledOrder {
    pub price: Price,
    pub size: f64,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    // required for buy order -> asks limits -> sort by cheapest first.
    pub fn fetch_ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits: Vec<&mut Limit> = self.asks.values_mut().collect();
        limits.sort_by_key(|limit| limit.price);
        limits
    }

    // required for sell order -> bid limits -> sort by highest first
    pub fn fetch_bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits: Vec<&mut Limit> = self.bids.values_mut().collect();
        limits.sort_by_key(|limit| limit.price);
        limits.reverse(); // Reverse to get highest first
        limits
    }

    pub fn add_limit_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.order_type {
            OrderType::Ask => {
                let limit = self.asks.get_mut(&price);
                match limit {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
            OrderType::Bid => {
                let limit = self.bids.get_mut(&price);
                match limit {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
        }
    }

    pub fn fill_market_order(&mut self, order: &mut Order) -> FillResult {
        let limits = match order.order_type {
            OrderType::Ask => self.fetch_bid_limits(),
            OrderType::Bid => self.fetch_ask_limits(),
        };

        let mut filled_size = 0.0;
        let mut total_value = 0.0;
        let mut filled_orders = Vec::new();

        for limit_order in limits {
            let limit_filled = limit_order.fill_order_with_tracking(order, &mut filled_orders);
            filled_size += limit_filled;
            total_value += limit_filled * limit_order.price.to_f64();

            if order.is_filled() {
                break;
            }
        }

        let avg_fill_price = if filled_size > 0.0 {
            total_value / filled_size
        } else {
            0.0
        };
        let remaining_size = order.size;

        FillResult {
            filled_size,
            remaining_size,
            avg_fill_price,
            filled_orders,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integral,
            fractional,
            scalar,
        }
    }

    fn to_f64(&self) -> f64 {
        (self.integral as f64) + (self.fractional as f64) / self.scalar as f64
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn get_liquidity(&self) -> f64 {
        self.orders.iter().map(|order| order.size).sum()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }

        if !market_order.is_filled() {
            {}
        }
    }

    fn fill_order_with_tracking(
        &mut self,
        market_order: &mut Order,
        filled_orders: &mut Vec<FilledOrder>,
    ) -> f64 {
        let mut filled_size = 0.0;
        for limit_order in self.orders.iter_mut() {
            let fill_size = if market_order.size < limit_order.size {
                market_order.size
            } else {
                limit_order.size
            };
            market_order.size -= fill_size;
            limit_order.size -= fill_size;
            filled_size += fill_size;

            if market_order.is_filled() {
                break;
            }
        }

        if filled_size > 0.0 {
            filled_orders.push(FilledOrder {
                price: self.price,
                size: filled_size,
            });
        }
        filled_size
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    order_type: OrderType,
}

impl Order {
    pub fn new(order_type: OrderType, size: f64) -> Order {
        Order { size, order_type }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn orderbook_fill_market_order_ask() {
        let mut orderbook = OrderBook::new();

        let buy_limit_order1 = Order::new(OrderType::Bid, 40.0);
        let buy_limit_order2 = Order::new(OrderType::Bid, 35.0);

        orderbook.add_limit_order(1000.0, buy_limit_order1);
        orderbook.add_limit_order(990.0, buy_limit_order2);

        let mut sell_order = Order::new(OrderType::Ask, 30.0);

        let fill_result = orderbook.fill_market_order(&mut sell_order);
        let bid_limits = orderbook.fetch_bid_limits();
        let ask_limits = orderbook.fetch_ask_limits();
        print!("{:?}", ask_limits);

        assert_eq!(2, 5);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_1 = Order::new(OrderType::Ask, 38.0);
        let buy_limit_order_2 = Order::new(OrderType::Ask, 62.0);
        limit.add_order(buy_limit_order_1);
        limit.add_order(buy_limit_order_2);

        let mut sell_market_order = Order::new(OrderType::Bid, 40.0);
        let mut sell_market_order_2 = Order::new(OrderType::Bid, 40.0);
        let fill_result = limit.fill_order_with_tracking(&mut sell_market_order, &mut Vec::new());
        let fill_result_2 =
            limit.fill_order_with_tracking(&mut sell_market_order_2, &mut Vec::new());

        assert_eq!(sell_market_order.size, 0.0);
        assert_eq!(sell_market_order_2.size, 0.0);
        println!("{:?}", limit);
        assert_eq!(limit.orders[0].size, 20.0);
    }

    #[test]
    fn market_order_semi_fill() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_1 = Order::new(OrderType::Ask, 20.0);
        let buy_limit_order_2 = Order::new(OrderType::Ask, 20.0);
        limit.add_order(buy_limit_order_1);
        limit.add_order(buy_limit_order_2);

        let mut sell_market_order = Order::new(OrderType::Bid, 50.0);
        let fill_result = limit.fill_order_with_tracking(&mut sell_market_order, &mut Vec::new());
        assert_eq!(sell_market_order.size, 10.0);
        println!("{:?}", limit);
        assert_eq!(limit.orders[0].size, 0.0);
        assert_eq!(limit.orders[1].size, 0.0);
        assert_eq!(sell_market_order.is_filled(), false);
    }

    #[test]
    fn test_total_volumes() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_1 = Order::new(OrderType::Ask, 20.0);
        let buy_limit_order_2 = Order::new(OrderType::Ask, 20.0);
        limit.add_order(buy_limit_order_1);
        limit.add_order(buy_limit_order_2);

        let total_liquidity = limit.get_liquidity();
        assert_eq!(total_liquidity, 40.0);
    }
}

// what we are doing right now , we are filling the markets order with the limit orders
// no check for , is even enough volume available .
// what is the avg. price for which the market order has been filled .
// for limit order we'll check if there is enough volume availble , fill it , make order for the rest.
