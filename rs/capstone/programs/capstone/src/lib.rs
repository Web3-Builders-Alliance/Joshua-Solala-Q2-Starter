mod group;
mod user;

use anchor_lang::prelude::*;

// Use Contexts
use crate::{group::contexts::*, user::context::*, group::states::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod capstone {
    use super::*;

    // Initialize User
    pub fn create_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.group_count = 0;
        Ok(())
    }

    // Group Functionalities
    // Create Group
    pub fn create_group(
        ctx: Context<CreateGroup>,
        name_of_group: String,
        duration_of_contribution: u8,
        contribution_amount: u128,
        group_description: String,
        group_terms: String,
        max_number: u8,
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let savings_group = &mut ctx.accounts.saving_group;
        let savings_groups_list = &mut ctx.accounts.savings_groups;

        let new_savings_group = SavingsGroup {
            owner: user_profile.authority.key(),
            name_of_group,
            duration_of_contribution,
            contribution_amount,
            group_description,
            group_terms,
            max_number,
            total_money: 0,
            active: false,
            account: savings_group.to_account_info().key(),
        };

        // Increment user group counts
        user_profile.group_count = user_profile.group_count.checked_add(1).unwrap();

        // Push savings group to groups list
        savings_groups_list.savings_groups.push(new_savings_group);
        Ok(())
    }
}
