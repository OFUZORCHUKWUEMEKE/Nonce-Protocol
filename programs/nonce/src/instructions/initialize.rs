use crate::{
    constants::*,
    errors::*,
    state::{ProtocolVault, SavingsAccount, SavingsType},
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use anchor_spl::{associated_token::get_associated_token_address, token_interface};

#[derive(Accounts)]
pub struct InitProtocolVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=signer,
        space= DISCRIMINATOR + ProtocolVault::INIT_SPACE,
        seeds=[b"protocol"],
        bump
    )]
    pub protocol_sol_vault: Account<'info, ProtocolVault>,
    #[account(
        init,
        seeds=[b"protocol"],
        bump,
        payer=payer,
        token::mint = mint,
        token::token_program = token_program,
        token::authority = protocol_sol_vault,
    )]
    pub protocol_usdc_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct InitializeSavings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes(), savings_type.try_to_vec()?.as_slice()],
        bump,
        payer=signer,
        space=DISCRIMINATOR + SavingsAccount::INIT_SPACE
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(
    ctx: Context<InitializeSavings>,
    name: String,
    description: String,
    savings_type: SavingsType,
    is_sol: bool,
    amount: u64,
    lock_duration: Option<i64>,
    unlock_price: Option<u64>,
) -> Result<()> {
    let savings_account = &mut ctx.accounts.savings_account;
    savings_account.name = name;
    savings_account.description = description;
    savings_account.savings_type = savings_type;
    savings_account.is_sol = is_sol;
    savings_account.owner = ctx.accounts.signer.key();
    savings_account.bump = ctx.bumps.savings_account;
    savings_account.created_at = Clock::get()?.unix_timestamp;
    if savings_account.amount > 0 {
        let new = savings_account.amount.checked_add(amount);
        savings_account.amount = new.unwrap();
    } else {
        savings_account.amount = amount;
    }
    if lock_duration.is_some() {
        savings_account.lock_duration = lock_duration.unwrap();
    } else {
        savings_account.lock_duration = 0
    }
    if unlock_price.is_some() {
        savings_account.unlock_price = unlock_price.unwrap();
    } else {
        savings_account.unlock_price = 0;
    }
    Ok(())
}

pub fn initialize_protocol(ctx: Context<InitProtocolVault>) -> Result<()> {
    let protocol_vault = &mut ctx.accounts.protocol_sol_vault;
    protocol_vault.authority = ctx.accounts.signer.key();
    protocol_vault.total_sol_saved = 0;
    protocol_vault.total_usdc_saved = 0;
    protocol_vault.last_updated = Clock::get()?.unix_timestamp;

    Ok(())
}
