use anchor_lang::prelude::*;

#[error]
pub enum MyMarketError {
    #[msg("InvalidProduct")]
    InvalidProduct,
    #[msg("InsufficientFunds")]
    InsufficientFunds,
    // Другие пользовательские ошибки
}
