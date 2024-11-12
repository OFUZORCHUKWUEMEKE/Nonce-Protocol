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
        seeds=[b"AUTH"],
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
        space=SavingsAccount::INIT_SPACE
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    // #[account(
    //     mut,
        // constraint = if !is_sol{
        //     let ata = get_associated_token_address(&signer.key(),&usdc_mint.as_ref().unwrap().key());
        //     token_account.key() == ata
        // }else{
        //     true
        // }
    // )]
    // pub token_account: Option<Interface<'info, TokenInterface>>,
    // #[account(
    //     init,
    //     token::mint = usdc_mint,
    //     token::authority=savings_account,
    //     payer=signer,
    //     seeds=[b"token_account",name.as_bytes(),signer.key().as_ref(),description.as_bytes(), savings_type.try_to_vec()?.as_slice()],
    //     bump
    // )]
    // pub user_usdc_savings_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    // pub usdc_mint: Option<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    // pub associated_token_program: Interface<'info, token_interface::TokenInterface>,
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
    }else{
        savings_account.amount = amount;
    }
    if lock_duration.is_some(){
        savings_account.lock_duration = lock_duration.unwrap();
    }else{
        savings_account.lock_duration = 0
    }
    Ok(())
}
