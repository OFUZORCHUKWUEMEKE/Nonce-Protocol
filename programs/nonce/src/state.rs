use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SavingsAccount{
    #[max_len(20)]
    pub name:String,
    pub sol_amount:Option<u64>,
    pub usdc_amount:Option<f64>,
    #[max_len(50)]
    pub description:String,
    pub owner:Pubkey,
    pub bump:u8,
    pub is_active:bool,
    pub lock_duration:Option<i64>,
    pub created_at:i64,
    pub savings_type:SavingsType,
    pub total_sol_saved:u64,
    pub total_usdc_saved:u64
}

#[derive(AnchorDeserialize,AnchorSerialize,PartialEq,Eq,Clone)]
#[derive(InitSpace)]
pub enum SavingsType{
    TimeLockedSavings,
    PriceLockedSavings
}