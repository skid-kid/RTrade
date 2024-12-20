use std::collections::HashMap;

#[derive(Debug)]
pub enum BidorAsk {
    Bid,
    Ask,
}
#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}
impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidorAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidorAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit: Limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}
impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}
#[derive(Debug)]
pub struct Limit {
    price: Price,
    order: Vec<Order>,
}
impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }
}
#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidorAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidorAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
}
