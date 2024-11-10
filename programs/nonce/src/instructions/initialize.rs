use anchor_lang::prelude::*;
use crate::state::{SavingsAccount,SavingsType};

#[derive(Accounts)]
#[instructions(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
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
    #[account(
        mut,
        constraint = if !is_sol{
            let ata = get_associated_token_address(&owner.key(),&usdc_mint.as_ref().unwrap().key());
            token_account.as_ref().unwrap().key()
        }else{
            true
        }
    )]
    pub token_account:Option<InterfaceAccount<'nfo,token_program::TokenAccount>>
    pub usdc_mint:Option<InterfaceAccount<'info,token_interface::Mint>>,
    pub token_program:InterfaceAccount<'info,token_interface::TokenInterface>,

}