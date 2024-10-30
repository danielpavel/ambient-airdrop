use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Token}, token_interface::{Mint, TokenAccount}};

use crate::state::AirdropState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    /// CHECK: envorce an already initialized mint account
    mint: InterfaceAccount<'info, Mint>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + AirdropState::INIT_SPACE,
        seeds = [b"airdrop", mint.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    airdrop_state: Account<'info, AirdropState>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = airdrop_state
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, bumps: &InitializeBumps) {
        self.airdrop_state.set_inner( AirdropState { 
            owner: self.signer.key(),
            bump: bumps.airdrop_state
        });
    }
}


