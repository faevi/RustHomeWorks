mod products;
mod sellers;
mod buyers;
mod admins;
mod common;

use anchor_lang::prelude::*;
use crate::products::inventory::MyInventory;
use crate::products::pricing::Pricing;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::sellers::Sellers;
use crate::buyers::Buyers;
use crate::admins::Admins;
use crate::common::Common;


declare_id!("9vRQ68z2Mz5eZkneiwTDMni9AHtfCEV72HRgQgY7Jsap");

#[account]
#[derive(Default)]
pub struct Market {
pub inventory: MyInventory,
pub pricing: Pricing,
pub sellers: Sellers,
pub buyers: Buyers,
pub admins: Admins,
pub common: Common,
}

#[program]
pub mod my_market {
use super::*;

pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    // Инициализация вашего умного контракта
    let market = &mut ctx.accounts.market;
    market.inventory = MyInventory::new();
    market.pricing = Pricing::new();
    market.sellers = Sellers::new();
    market.buyers = Buyers::new();
    market.admins = Admins::new();
    market.common = Common::new();

    Ok(())
}




#[access_control]
pub fn add_product(ctx: Context<instruction::AddProduct>, product: crate::products::inventory::Product) -> ProgramResult {
    // Добавление продукта в инвентарь
    let market = &mut ctx.accounts.market;
    market.inventory.add_product(product)?;

    Ok(())
}


#[account(init, payer = user)]
pub struct Initialize<'info> {
    pub market: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

}


