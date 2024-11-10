use anchor_lang::prelude::*;

declare_id!("4XSnD5eXrLPEuvEmN9hieF3ywBKXKPU2hogH7fUKYxmF");

#[program]
pub mod nonce {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
