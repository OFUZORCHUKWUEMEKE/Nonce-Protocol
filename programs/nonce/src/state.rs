use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct ProtocolVault{
    pub authority:Pubkey,
    pub total_sol_saved:u64,
    pub total_usdc_saved:u64,
    pub last_updated:i64,
    pub mint_address:Pubkey
}

#[account]
#[derive(InitSpace)]
pub struct SavingsAccount{
    #[max_len(20)]
    pub name:String,
    pub amount:u64,
    #[max_len(50)]
    pub description:String,
    pub owner:Pubkey,
    pub bump:u8,
    pub is_active:bool,
    pub lock_duration:i64,
    pub created_at:i64,
    pub savings_type:SavingsType,
    pub is_sol:bool,
}

#[derive(AnchorDeserialize,AnchorSerialize,PartialEq,Eq,Clone)]
#[derive(InitSpace)]
pub enum SavingsType{
    TimeLockedSavings,
    PriceLockedSavings
}