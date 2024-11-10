use anchor_lang::prelude::*;
use crate::state::{SavingsAccount};

#[derive(Accounts)]
#[instructions(name:String,description:String,savings_type:SavingsType)]
pub struct InitializeSavings<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(
        init,
        seeds=[name.as_bytes().as_ref(),signer.key().as_ref(),description.as_ref(),savings_type.to_bytes()],
        bump,
        payer=signer,
        space=SavingsAccount::INIT_SPACE
    )]
    pub savings_account:Account<'info,SavingsAccount>,
    pub usdc_mint:InterfaceAccount<'info,token_interface::Mint>,
    pub token_program:InterfaceAccount<'info,token_interface::TokenInterface>,

}