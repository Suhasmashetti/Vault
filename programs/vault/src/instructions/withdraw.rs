use anchor_lang::prelude::*;
use anchor_spl::token::{self, TransferChecked, Token, TokenAccount, Mint};
use crate::state::VaultDataAccount;
use crate::error::VaultError;
use crate::vault;

pub fn withdraw_tokens(ctx: Context<WithdrawStruct>, amount: u64) -> Result<()> {

    let vault_data = &ctx.accounts.vault_data;
    let seeds = &[
        b"vault_data",
        vault_data.depositer.as_ref(),
        vault_data.recipient.as_ref(),
        &vault_data.seed.to_le_bytes(),
        &[vault_data.vault_bump],
    ];
    let signer = &[&seeds[..]];
    token::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.vault_data.to_account_info(),
            },
            seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawStruct<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,
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
        associated_token::authority = vault_data,
        associated_token::mint = mint,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = recipient,
        associated_token::authority = recipient,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}