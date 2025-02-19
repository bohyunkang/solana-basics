use anchor_lang::prelude::*;

declare_id!("CuQuUB6yApxjPSjuB9UZc6ZyJfSk8tVAnqfnH6pFgEnA");

#[program]
pub mod my_first_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
