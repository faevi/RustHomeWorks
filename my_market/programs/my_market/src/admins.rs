use std::collections::HashSet;
use anchor_lang::prelude::Pubkey;

pub struct Admins {
    pub admins: HashSet<Pubkey>,
}

impl Admins {
    pub fn new() -> Self {
        Self {
            admins: HashSet::new(),
        }
    }

    pub fn add_admin(&mut self, admin_key: Pubkey) {
        self.admins.insert(admin_key);
    }

    pub fn is_admin(&self, admin_key: &Pubkey) -> bool {
        self.admins.contains(admin_key)
    }
}