use anchor_lang::prelude::*;
use 
    anchor_spl::{associated_token::AssociatedToken, 
    token::{close_account, CloseAccount}, 
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{error::VaultError, state::VaultDataAccount};


pub fn withdraw_tokens(ctx: Context<WithdrawStruct>) -> Result<()> {
    let vault_data = &ctx.accounts.vault_data;

    let seeds: &[&[u8]] = &[
        b"vault_data",
        vault_data.depositer.as_ref(),
        vault_data.recipient.as_ref(),
        &vault_data.seed.to_le_bytes(),
        &[vault_data.vault_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let current_time = Clock::get()?.unix_timestamp;
    require!(current_time >= vault_data.unlock_time, VaultError::VaultLocked);
    require!(vault_data.total_amount > 0, VaultError::VaultEmpty);

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.vault_data.to_account_info(),
            },
            signer_seeds,
        ),
        vault_data.total_amount,
        ctx.accounts.mint.decimals,
    )?;

    close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.vault_token_account.to_account_info(),
                destination: ctx.accounts.depositer.to_account_info(),
                authority: ctx.accounts.vault_data.to_account_info(),
            },
            signer_seeds,
        ),
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawStruct<'info> {

    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(mut)]
    pub depositer: SystemAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = depositer,
        seeds = [b"vault_data", depositer.key().as_ref(), recipient.key().as_ref(), &vault_data.seed.to_le_bytes()],
        bump = vault_data.vault_bump,
        has_one = depositer,
        has_one = recipient,
        has_one = mint,
    )]
    pub vault_data: Account<'info, VaultDataAccount>,

    #[account(
        mut,
        token::authority = vault_data,
        token::mint = mint,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = recipient,
        associated_token::authority = recipient,
        associated_token::mint = mint,
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}