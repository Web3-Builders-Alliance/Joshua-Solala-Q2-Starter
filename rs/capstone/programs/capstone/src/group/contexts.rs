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
        seeds = [CREATE_GROUP, authority.key().as_ref(), &[user_profile.group_count]],
        bump,
        payer = authority,
        space = 2865 + 8,
    )]
    pub saving_group: Box<Account<'info, SavingsGroup>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub savings_groups: Box<Account<'info, SavingsGroupList>>,
    pub system_program: Program<'info, System>
}

// Get all groups
#[derive(Accounts)]
#[instruction()]
pub struct GetGroups<'info> {
    #[account(mut)]
    pub savings_groups_list: Box<Account<'info, SavingsGroupList>>
}