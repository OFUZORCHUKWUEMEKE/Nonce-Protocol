mod state;
mod errors;
mod constants;
mod instructions;

use instructions::*;
use anchor_lang::prelude::*;

declare_id!("4XSnD5eXrLPEuvEmN9hieF3ywBKXKPU2hogH7fUKYxmF");

#[program]
pub mod nonce {
    use instructions::InitProtocolVault;

    use super::*;

    pub fn initialize_protocol(ctx:Context<InitProtocolVault>)->Result<()>{
        Ok(())
    }

    pub fn initialize_savings(ctx:Context<InitializeSavings>)->Result<()>{
        Ok(())
    }

    pub fn deposit(ctx:Context<Deposit>)->Result<()>{
        Ok(())
    }

}

