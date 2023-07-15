use anchor_lang::prelude::*;

declare_id!("9vRQ68z2Mz5eZkneiwTDMni9AHtfCEV72HRgQgY7Jsap");

#[program]
pub mod my_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
