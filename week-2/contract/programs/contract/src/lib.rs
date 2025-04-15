use anchor_lang::prelude::*;

declare_id!("Dhu5CEfrrsbWvJKUn3iufSneNNNtsCPC42WptuT8QxrP");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
