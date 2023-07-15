use anchor_lang::prelude::*;
use my_market::my_market::MyMarket;

fn main() {
    // Инициализация Anchor
    let program = anchor_lang::solana_program::program::entrypoint::<MyMarket>();
    program::invoke_entrypoint(program);
}
