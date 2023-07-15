use std::collections::HashMap;
use anchor_lang::prelude::Pubkey;

pub struct Buyers {
    pub buyers: HashMap<Pubkey, Buyer>,
}

impl Buyers {
    pub fn new() -> Self {
        Self {
            buyers: HashMap::new(),
        }
    }

    pub fn add_buyer(&mut self, buyer: Buyer) {
        self.buyers.insert(buyer.public_key, buyer);
    }

    pub fn get_buyer(&self, buyer_key: &Pubkey) -> Option<&Buyer> {
        self.buyers.get(buyer_key)
    }
}

pub struct Buyer {
    pub public_key: Pubkey,
    pub name: String,
    pub balance: u64,
}

impl Buyer {
    pub fn new(public_key: Pubkey, name: String, balance: u64) -> Self {
        Self {
            public_key,
            name,
            balance,
        }
    }
}