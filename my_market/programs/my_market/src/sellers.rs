use std::collections::HashMap;
use anchor_lang::prelude::Pubkey;

pub struct Sellers {
    pub sellers: HashMap<Pubkey, Seller>,
}

impl Sellers {
    pub fn new() -> Self {
        Self {
            sellers: HashMap::new(),
        }
    }

    pub fn add_seller(&mut self, seller: Seller) {
        self.sellers.insert(seller.public_key, seller);
    }

    pub fn get_seller(&self, seller_key: &Pubkey) -> Option<&Seller> {
        self.sellers.get(seller_key)
    }
}

pub struct Seller {
    pub public_key: Pubkey,
    pub name: String,
}

impl Seller {
    pub fn new(public_key: Pubkey, name: String) -> Self {
        Self {
            public_key,
            name,
        }
    }
}
