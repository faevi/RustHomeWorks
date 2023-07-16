use anchor_lang::prelude::*;
use my_market::my_market::Market;

fn main() {
    // Инициализация Anchor
    let program = anchor_lang::solana_program::program::entrypoint::<Market>();
    program::invoke_entrypoint(program);
}
