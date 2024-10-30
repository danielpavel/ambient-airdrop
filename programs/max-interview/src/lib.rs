use anchor_lang::prelude::*;

declare_id!("7fZPsG5cvQKWTfjh4KXedyxmNfFQuGEqCsGezqoECETj");

mod context;
mod state;

pub use context::*;

#[program]
pub mod max_interview {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps);

        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>, amount: u64) -> Result {
        ctx.accounts.airdrop(amount);

        Ok(())
    }
}
