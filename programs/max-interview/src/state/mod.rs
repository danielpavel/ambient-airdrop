use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AirdropState {
    pub owner: Pubkey,
    pub bump: u8,
}
