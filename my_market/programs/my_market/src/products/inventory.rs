use anchor_lang::prelude::Pubkey;
use ::borsh::{BorshSerialize, BorshDeserialize};
use anchor_lang::prelude::*;


#[derive(BorshSerialize, BorshDeserialize)]
pub struct MyInventory {
    pub products: Vec<Product>,
}

impl MyInventory {
    pub fn new() -> Self {
        Self {
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn get_products(&self) -> &[Product] {
        &self.products
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub owner: Pubkey,
}

impl Product {
    pub fn new(id: u64, name: String, price: u64, owner: Pubkey) -> Self {
        Self {
            id,
            name,
            price,
            owner,
        }
    }
}
