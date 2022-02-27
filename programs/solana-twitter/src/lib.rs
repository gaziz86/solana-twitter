use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("65rXkFFfKUZc6uN8NSSmc8gjax9aN6wMRi7NoVC1LRt7");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
