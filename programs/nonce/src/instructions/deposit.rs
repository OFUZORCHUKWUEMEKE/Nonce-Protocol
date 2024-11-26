use crate::{
    constants::*,
    errors::*,
    state::{ProtocolState, SavingsAccount, SavingsType},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{AssociatedToken},
    token::{self, Token},
    token_interface::{self, Mint, TokenAccount, TransferChecked},
};

// Nice One Bro God will Help You

#[derive(Accounts)]
#[instruction(name:String,description:String,savings_type:SavingsType,is_sol:bool)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[name.as_bytes(),signer.key().as_ref(),description.as_bytes()],
        bump= savings_account.bump
    )]
    pub savings_account: Account<'info, SavingsAccount>,
    #[account(
        mut,
        seeds=[b"vault",signer.key().as_ref()],
        bump
    )]
    pub token_vault_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(mut)]
    pub protocol_state: Account<'info, ProtocolState>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(
    ctx: Context<Deposit>,
    _name: String,
    _description: String,
    savings_type: SavingsType,
    is_sol: bool,
    amount: u64,
    time_lock: Option<i64>,
    unlock_price: Option<u64>,
) -> Result<()> {
    let vault_sol_account = &mut ctx.accounts.savings_account;
    let protocol_state = &mut ctx.accounts.protocol_state;
    // let savings_account = &mut ctx.accounts.savings_account;
    match savings_type {
        SavingsType::TimeLockedSavings => {
            if is_sol == true {
                let cpi_ctx = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.signer.to_account_info(),
                        to: vault_sol_account.to_account_info(),
                    },
                );

                anchor_lang::system_program::transfer(cpi_ctx, amount);
                protocol_state.total_sol_saved.checked_add(amount);
            } else {
                let transfer_cpi_accounts = TransferChecked {
                    from: ctx.accounts.user_ata.to_account_info(),
                    to: ctx.accounts.token_vault_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
                let decimals = ctx.accounts.mint.decimals;

                token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
                protocol_state.total_usdc_saved.checked_add(amount);
            }
        }
        _ => {
            if is_sol == true {
                let cpi_ctx = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.signer.to_account_info(),
                        to: vault_sol_account.to_account_info(),
                    },
                );
                anchor_lang::system_program::transfer(cpi_ctx, amount);
                protocol_state.total_sol_saved.checked_add(amount);
            } else {
                let transfer_cpi_accounts = TransferChecked {
                    from: ctx.accounts.user_ata.to_account_info(),
                    to: ctx.accounts.token_vault_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
                let decimals = ctx.accounts.mint.decimals;

                token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
                protocol_state.total_usdc_saved.checked_add(amount);
            }
            println!("PriceLocked");
        }
    }
    Ok(())
}
