use anchor_lang::prelude::*;

declare_id!("UdWcvPo4mu1kYjuaUgCbH3XCeehN9NWAukB97Dc8YBE");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
