use anchor_lang::prelude::*;

use crate::user::{
    constants::*,
    states::*
};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 32 + 1 + 1 + 8
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,
    pub system_program: Program<'info, System>
}