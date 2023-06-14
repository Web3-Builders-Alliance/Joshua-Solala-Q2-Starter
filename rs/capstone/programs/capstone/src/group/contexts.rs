use anchor_lang::prelude::*;
// Use States
use crate::group::{states::*, constants::*};
use crate::user::{states::*, constants::*};

// Group State Context
#[derive(Accounts)]
#[instruction()]
pub struct CreateGroup<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [CREATE_GROUP, authority.key().as_ref(), &[user_profile.last_group]],
        bump,
        payer = authority,
        space = 2865 + 8,
    )]
    pub saving_group: Box<Account<'info, SavingsGroup>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}