use crate::state::{SavingsAccount, SavingsType};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

#[derive(Accounts)]
#[instructions(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct InitializeSavings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes(), savings_type.try_to_vec()?.as_slice()],
        bump,
        payer=signer,
        space=SavingsAccount::INIT_SPACE
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(
        mut,
        constraint = if !is_sol{
            let ata = get_associated_token_address(&owner.key(),&usdc_mint.as_ref().unwrap().key());
            token_account.as_ref().unwrap().key()
        }else{
            true
        }
    )]
    pub token_account: Option<InterfaceAccount<'nfo, token_program::TokenAccount>>,
    pub usdc_mint: Option<InterfaceAccount<'info, token_interface::Mint>>,
    pub token_program: InterfaceAccount<'info, token_interface::TokenInterface>,
    pub system_program: Progrm<'info, System>,
}

pub fn initialize(
    ctx: Context<InitializeSavings>,
    name: String,
    description: String,
    savings_type: SavingsType,
    is_sol: bool,
    sol_amount: Option<u64>,
    usdc_amount: Option<i64>,
    lock_duration: Option<i64>,
) -> Result<()> {
    let savings_account = &mut ctx.accounts.savings_account;
    savings_account.name = name;
    savings_account.description = description;
    savings_account.savings_type = savings_type;
    savings_account.is_sol = is_sol;
    savings_account.owner = ctx.accounts.owner.key();
    savings_account.bump = *ctx.bumps.get("savings_account").unwrap();
    savings_account.created_at = Clock::get()?.unix_timestamp;
    savings_account.sol_amount = Some(
        savings_account
            .sol_amount
            .unwrap_or(0) // If None, use 0
            .checked_add(sol_amount.unwrap_or(0)) // Safe addition
            .ok_or(NonceSavingsError::Overflow)?, // Handle overflow
    );
    savings_account.usdc_amount = Some(
        savings_account
            .usdc_amount
            .unwrap_or(0)
            .checked_add(usdc_amount.unwrap_or(0))
            .ok_or(NonceSavingsError::Overflow)?,
    );
    savings_account.total_sol_saved = Some(
        savings_account
            .total_sol_saved
            .unwrap_or(0)
            .checked_add(total_sol_saved.unwrap_or(0))
            .ok_or(NonceSavingsError::Overflow),
    );
    savings_account.total_usdc_saved = 0;
    match lock_duration{
        Some(duration) =>savings_account.lock_duration = Some(duration),
        None => savings_account.lock_duration = None
    }
}
