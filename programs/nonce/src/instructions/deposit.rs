use crate::{state::{SavingsAccount, SavingsType},errors::*,constants::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{AssociatedToken,get_associated_token_address}, token::{self, Token}, token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked}
};


#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes(), savings_type.try_to_vec()?.as_slice()],
        bump= savings_account.bump
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    pub usdc_mint: Option<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = usdc_mint,
        associated_token::authority = signer,
        constraint = if !is_sol{
            let ata = get_associated_token_address(&signer.key(),&usdc_mint.as_ref().unwrap().key());
            user_ata.key() == ata
        }else{
            true
        }
    )]
    pub user_ata:Option<InterfaceAccount<'info, TokenAccount>> ,
    pub token_program:Option<Interface<'info, token_interface::TokenInterface>>,
    pub associated_token_program:Option<Program<'info,AssociatedToken>>,
    pub system_program: Program<'info, System>,
}


