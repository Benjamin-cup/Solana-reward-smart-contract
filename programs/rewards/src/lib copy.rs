use anchor_lang::prelude::*;

declare_id!("AAv2GZesLSUJm2Q63prFfkztTYEgzXLWfTfBJygC2dW5");

#[program]
pub mod rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
