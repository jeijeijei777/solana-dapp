use anchor_lang::prelude::*;

declare_id!("A8EHVhZtUGUfThAdbMkCc7c2yAbt18SRMhXfwjHmG7c5");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
