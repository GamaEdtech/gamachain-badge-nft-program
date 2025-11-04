use anchor_lang::prelude::*;

declare_id!("4mxsUQvUS7sComPqovgyrf4cBYeiRK7wmbofNBvnmzGn");

#[program]
pub mod gamachain_badge_nft_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
