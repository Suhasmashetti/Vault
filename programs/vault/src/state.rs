use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultDataAccount {
    pub is_initialized: bool,
    pub total_amount: u64,
    pub depositer: Pubkey,
    pub recipient: Pubkey,
    pub time_of_deposit: i64,
    pub unlock_time: i64,
    pub seed: u64,
    pub vault_bump: u8, 
    
}