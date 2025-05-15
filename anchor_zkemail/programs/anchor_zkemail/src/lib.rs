use anchor_lang::prelude::*;

declare_id!("AqSt8YPVacLGQQc2AqXVmuKgEzKk7dqws7DscqoxwWWD");

#[program]
pub mod anchor_zkemail {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
