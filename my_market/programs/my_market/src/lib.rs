#[program]
pub mod market {
    use anchor_lang::prelude::*;

    #[state]
    pub struct Market {
        pub products: Vec<Product>,
    }

    impl Market {
        pub fn new(ctx: Context<Empty>) -> Result<Self> {
            Ok(Self {
                products: vec![],
            })
        }

        pub fn create_product(&mut self, ctx: Context<Empty>, name: String, price: u64, quantity: u64) -> Result<()> {
            let product = Product {
                name,
                price,
                quantity,
            };
            self.products.push(product);
            Ok(())
        }

        // Другие функции для покупки, обновления и т.д.
    }

    #[derive(Accounts)]
    pub struct Empty {}

    #[account]
    pub struct Product {
        pub name: String,
        pub price: u64,
        pub quantity: u64,
    }
}
