use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("The vault is not yet unlocked.")]
    VaultLocked,
    #[msg("The vault is empty.")]
    VaultEmpty,
    #[msg("Invalid amount specified.")]
    InvalidAmount,
    #[msg("The vault is not initialized.")]
    VaultNotInitialized,
    #[msg("Unauthorized action.")]
    Unauthorized,
    #[msg("Invalid unlock time specified.")]
    InvalidUnlockTime,
}