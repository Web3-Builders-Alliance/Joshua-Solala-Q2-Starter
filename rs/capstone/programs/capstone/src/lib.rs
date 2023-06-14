mod group;
mod user;

use anchor_lang::prelude::*;

// Use Contexts
use crate::group::{
    contexts::*
};
// Use User Context
use crate::user::{
    context::*
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod capstone {
    use super::*;

    pub fn create_group(ctx: Context<InitializeUser>) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.group_count = 0;
        user_profile.last_group = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
