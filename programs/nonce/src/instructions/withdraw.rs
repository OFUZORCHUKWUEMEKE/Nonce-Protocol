use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{get_associated_token_address, AssociatedToken},
    token_interface::{self, Mint, TokenAccount, TransferChecked},
};
use pyth_solana_receiver_sdk::{
    cpi,
    price_update::{get_feed_id_from_hex, PriceUpdateV2},
};

use crate::{
    constants::{MAXIMUM_AGE, SOL_USD_FEED_ID, USDC_USD_FEED_ID},
    errors::NonceError,
    state::{ProtocolVault, SavingsAccount, SavingsType},
};

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes(), savings_type.try_to_vec()?.as_slice()],
        bump= savings_account.bump
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds=[b"protocol"],
        bump
    )]
    pub protocol_sol_vault: Account<'info, ProtocolVault>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = protocol_sol_vault,
        constraint = if !is_sol{
            let ata = get_associated_token_address(&signer.key(),&mint.key());
            user_ata.key() == ata
        }else{
            true
        }
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"AUTH"],
        bump,
        token::mint = mint,
        token::token_program = token_program,
        token::authority = protocol_sol_vault,
    )]
    pub protocol_usdc_vault: InterfaceAccount<'info, token_interface::TokenAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64, unlock_price: Option<u64>,lock_duration:Option<i64>) -> Result<()> {
    let savings_account = &mut ctx.accounts.savings_account;
    let price_update = &mut ctx.accounts.price_update;
    // let unlock_price = savings_account
    match savings_account.savings_type {
        SavingsType::PriceLockedSavings => {
            if savings_account.is_sol == true {
                let sol_feed_id = get_feed_id_from_hex(SOL_USD_FEED_ID);
                let sol_price = price_update.get_price_no_older_than(
                    &Clock::get()?,
                    MAXIMUM_AGE,
                    &sol_feed_id?,
                )?;
                let final_amount = (sol_price.price as u64).checked_mul(amount);
                if final_amount.unwrap() >= unlock_price.unwrap() {
                    let signer_seeds: &[&[&[u8]]] =
                        &[&[b"protocol", &[ctx.bumps.protocol_sol_vault]]];
                    let cpi_ctx = CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: ctx.accounts.protocol_sol_vault.to_account_info(),
                            to: ctx.accounts.signer.to_account_info(),
                        },
                    )
                    .with_signer(signer_seeds);
                    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
                } else {
                    return Err(NonceError::PriceNotReached.into());
                }
            } else {
                let sol_feed_id = get_feed_id_from_hex(USDC_USD_FEED_ID);
                let usdc_price = price_update.get_price_no_older_than(
                    &Clock::get()?,
                    MAXIMUM_AGE,
                    &sol_feed_id?,
                )?;
                let final_amount = (usdc_price.price as u64).checked_mul(amount);
                if final_amount.unwrap() >= unlock_price.unwrap() {
                    let signer_seeds: &[&[&[u8]]] =
                        &[&[b"protocol", &[ctx.bumps.protocol_sol_vault]]];
                    let cpi_program = ctx.accounts.token_program.to_account_info();
                    let mint_key = ctx.accounts.mint.key();
                    let decimals = ctx.accounts.mint.decimals;
                    let transfer_accounts = TransferChecked {
                        from: ctx.accounts.protocol_usdc_vault.to_account_info(),
                        to: ctx.accounts.user_ata.to_account_info(),
                        authority: ctx.accounts.protocol_sol_vault.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                    };
                    let ctx = CpiContext::new(cpi_program, transfer_accounts);
                    token_interface::transfer_checked(ctx, amount, decimals)?;
                }
            }
        }
        _ => {
            if savings_account.is_sol == true{
                let current_timestamp = Clock::get()?.unix_timestamp;
                if current_timestamp >= savings_account.created_at + lock_duration.unwrap(){
                    let signer_seeds: &[&[&[u8]]] =
                        &[&[b"protocol", &[ctx.bumps.protocol_sol_vault]]];
                    let cpi_ctx = CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: ctx.accounts.protocol_sol_vault.to_account_info(),
                            to: ctx.accounts.signer.to_account_info(),
                        },
                    )
                    .with_signer(signer_seeds);
                    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
                }else{
                    return Err(NonceError::FundsStillLocked.into());
                }
            }else{
                let current_timestamp = Clock::get()?.unix_timestamp;
                if current_timestamp >= savings_account.created_at + lock_duration.unwrap(){
                    let signer_seeds: &[&[&[u8]]] =
                        &[&[b"protocol", &[ctx.bumps.protocol_sol_vault]]];
                    let cpi_program = ctx.accounts.token_program.to_account_info();
                    let decimals = ctx.accounts.mint.decimals;
                    let transfer_accounts = TransferChecked {
                        from: ctx.accounts.protocol_usdc_vault.to_account_info(),
                        to: ctx.accounts.user_ata.to_account_info(),
                        authority: ctx.accounts.protocol_sol_vault.to_account_info(),
                        mint: ctx.accounts.mint.to_account_info(),
                    };
                    let ctx = CpiContext::new(cpi_program, transfer_accounts);
                    token_interface::transfer_checked(ctx, amount, decimals)?;
                }else{
                    return Err(NonceError::FundsStillLocked.into());
                }
            }
        }
    }

    Ok(())
}
