mod constants;
mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("4srKoX2V92mmfDVDGVHzTEw19Tku3uSoBxiER8isVzd7");

#[program]
pub mod nonce {
    use state::SavingsType;

    use super::*;
    /**
     * @Initialize Initialize Savings Account
     */
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

    /**
     * @deposit Deposit SOL and USDC
     */

    pub fn deposit(
        ctx: Context<Deposit>,
        name: String,
        description: String,
        savings_type: SavingsType,
        is_sol: bool,
        amount: u64,
        time_lock: Option<i64>,
        unlock_price: Option<u64>,
    ) -> Result<()> {
        deposit_handler(
            ctx,
            name,
            description,
            savings_type,
            is_sol,
            amount,
            time_lock,
            unlock_price,
        );
        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        unlock_price: Option<u64>,
        lock_duration: Option<i64>,
    ) -> Result<()> {
        withdraw_handler(ctx, amount, unlock_price, lock_duration);
        Ok(())
    }
}
