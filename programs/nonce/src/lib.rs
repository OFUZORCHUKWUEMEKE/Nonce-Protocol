mod constants;
mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("4XSnD5eXrLPEuvEmN9hieF3ywBKXKPU2hogH7fUKYxmF");

#[program]
pub mod nonce {
    use instructions::InitProtocolVault;
    use state::SavingsType;

    use super::*;

    pub fn initialize_protocol(ctx: Context<InitProtocolVault>) -> Result<()> {
        initialize_protocol(ctx);
        Ok(())
    }

    pub fn initialize_savings(
        ctx: Context<InitializeSavings>,
        name: String,
        description: String,
        savings_type: SavingsType,
        is_sol: bool,
        amount: u64,
        lock_duration: Option<i64>,
        unlock_price: Option<u64>,
    ) -> Result<()> {
        initialize(
            ctx,
            name,
            description,
            savings_type,
            is_sol,
            amount,
            lock_duration,
            unlock_price,
        );
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }
}
