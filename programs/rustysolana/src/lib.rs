use anchor_lang::prelude::*;

declare_id!("EzN6P2ZuFz8arhAHs59u5vSz2A5X2LFH6Cmb6iVzxX9g");

#[program]
pub mod rustysolana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
