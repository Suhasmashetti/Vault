use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Mint, Token, TokenAccount, Transfer},
    associated_token::AssociatedToken,
};
use crate::state::VaultDataAccount;

pub fn initialize_account(
    ctx: Context<DepositStruct>,
    amount: u64,
    unlock_time: i64,
    seed: u64,
) -> Result<()> {
    let vault_data = &mut ctx.accounts.vault_data;

    vault_data.is_initialized = true;
    vault_data.depositer = ctx.accounts.depositer.key();
    vault_data.recipient = ctx.accounts.recipient.key();
    vault_data.time_of_deposit = Clock::get()?.unix_timestamp;
    vault_data.total_amount = amount;
    vault_data.unlock_time = unlock_time;
    vault_data.vault_bump = ctx.bumps.vault_data;
    vault_data.seed = seed;

    Ok(())
}

pub fn deposit_tokens(ctx: Context<DepositStruct>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.depositer_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.depositer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DepositStruct<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        init,
        payer = depositer,
        space = 8 + VaultDataAccount::LEN,
        seeds = [b"vault_data", depositer.key().as_ref(), recipient.key().as_ref(), &seed.to_le_bytes()],
        bump,
    )]
    pub vault_data: Account<'info, VaultDataAccount>,

    #[account(
        init_if_needed,
        payer = depositer,
        associated_token::authority = vault_data,
        associated_token::mint = mint,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = depositer,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
    )]
    pub depositer_token_account: Account<'info, TokenAccount>,

    pub recipient: SystemAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
