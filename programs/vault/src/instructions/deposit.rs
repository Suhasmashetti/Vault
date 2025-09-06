use anchor_lang::prelude::*;
use 
    anchor_spl::{associated_token::AssociatedToken, 
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{error::VaultError, state::VaultDataAccount}; 

pub fn deposit_tokens(
    ctx: Context<DepositStruct>,
    amount: u64,
    unlock_time: i64,
    seed: u64,
) -> Result<()> {
    let vault_data = &mut ctx.accounts.vault_data;
    require!(amount > 0, VaultError::InvalidAmount);
    let now = Clock::get()?.unix_timestamp;
    require!(unlock_time > now, VaultError::InvalidUnlockTime);

    vault_data.is_initialized = true;
    vault_data.depositer = ctx.accounts.depositer.key();
    vault_data.recipient = ctx.accounts.recipient.key();
    vault_data.time_of_deposit = now;
    vault_data.unlock_time = unlock_time;
    vault_data.seed = seed;
    vault_data.mint = ctx.accounts.mint.key();
    vault_data.vault_bump = ctx.bumps.vault_data;
    vault_data.total_amount = amount;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositer_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.depositer.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DepositStruct<'info> {
    /// Signer depositing tokens
    #[account(mut)]
    pub depositer: Signer<'info>,

    /// Vault state PDA
    #[account(
        init,
        payer = depositer,
        space = 8 + VaultDataAccount::INIT_SPACE, // much cleaner than hardcoding
        seeds = [b"vault_data", depositer.key().as_ref(), recipient.key().as_ref(), &seed.to_le_bytes()],
        bump,
    )]
    pub vault_data: Account<'info, VaultDataAccount>,

    /// Vault token account (ATA owned by vault PDA)
    #[account(
        init_if_needed,
        payer = depositer,
        token::authority = vault_data,
        token::mint = mint,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Mint of the deposited token
    pub mint: InterfaceAccount<'info, Mint>,

    /// Depositer ATA
    #[account(
        mut,
        associated_token::authority = depositer,
        associated_token::mint = mint,
    )]
    pub depositer_token_account: InterfaceAccount<'info, TokenAccount>,

    /// Recipient
    pub recipient: SystemAccount<'info>,

    /// Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}