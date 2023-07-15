use std::collections::HashMap;

pub struct Pricing {
    pub prices: HashMap<u64, u64>,
}

impl Pricing {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
        }
    }

    pub fn set_price(&mut self, product_id: u64, price: u64) {
        self.prices.insert(product_id, price);
    }

    pub fn get_price(&self, product_id: u64) -> Option<&u64> {
        self.prices.get(&product_id)
    }
}
