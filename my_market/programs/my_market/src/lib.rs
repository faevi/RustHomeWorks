#[program]
pub mod market {
    use anchor_lang::prelude::*;

    #[state]
    pub struct Market {
        pub products: Vec<Product>,
        pub sellers: Vec<Pubkey>,
        pub admin: Pubkey,
    }

    impl Market {
        pub fn new(ctx: Context<Empty>) -> Result<Self> {
            Ok(Self {
                products: vec![],
                sellers: vec![],
                admin: *ctx.accounts.admin.key,
            })
        }

        pub fn create_product(&mut self, ctx: Context<CreateProduct>, name: String, price: u64, quantity: u64) -> Result<()> {
            let seller = &ctx.accounts.seller.key;
            let product = Product {
                name,
                price,
                quantity,
                seller: *seller,
            };
            self.products.push(product);
            Ok(())
        }

        pub fn buy_product(&mut self, ctx: Context<BuyProduct>, product_index: u32) -> Result<()> {
            let product = &mut self.products[product_index as usize];
            let buyer = &ctx.accounts.buyer.key;
            
            // Проверить достаточность средств у покупателя
            if ctx.accounts.buyer_token.amount < product.price {
                return Err(ErrorCode::InsufficientFunds.into());
            }

            // Проверить наличие товара
            if product.quantity == 0 {
                return Err(ErrorCode::OutOfStock.into());
            }

            // Перевести средства покупателя продавцу
            ctx.accounts
                .buyer_token
                .transfer(&mut ctx.accounts.seller_token, product.price)?;

            // Уменьшить количество товара
            product.quantity -= 1;

            Ok(())
        }

        // Другие функции для обновления цены, количества, удаления товаров и т.д.
    }

    #[derive(Accounts)]
    pub struct Empty {}

    #[derive(Accounts)]
    pub struct CreateProduct<'info> {
        #[account(init, payer = seller, space = 8 + 32 + 8 + 32)]
        pub product: Account<'info, Product>,
        #[account(signer)]
        pub seller: AccountInfo<'info>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    #[derive(Accounts)]
    pub struct BuyProduct<'info> {
        #[account(mut)]
        pub product: Account<'info, Product>,
        #[account(mut)]
        pub seller_token: Account<'info, TokenAccount>,
        #[account(mut, has_one = buyer)]
        pub buyer_token: Account<'info, TokenAccount>,
        #[account(signer)]
        pub buyer: AccountInfo<'info>,
    }

    #[account]
    pub struct Product {
        pub name: String,
        pub price: u64,
        pub quantity: u64,
        pub seller: Pubkey,
    }

    #[account]
    pub struct TokenAccount {
        pub amount: u64,
    }

    #[error]
    pub enum ErrorCode {
        #[msg("Insufficient funds")]
        InsufficientFunds,
        #[msg("Product is out of stock")]
        OutOfStock,
    }
}
