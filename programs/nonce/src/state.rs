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
    pub owner:PubKey,
    pub bump:u8,
    pub is_active:bool,
    pub lock_duration:164,
    pub created_at:i64,
    pub savings_type:SavingsType,
    pub total_sol_saved:u64,
    pub total_usdc_saved:u64
}

#[derive(AnchorDeserialize,AnchorSerialize,PartialEq,Eq)]
pub enum SavingsType{
    TimeLockedSavings,
    PriceLockedSavings
}