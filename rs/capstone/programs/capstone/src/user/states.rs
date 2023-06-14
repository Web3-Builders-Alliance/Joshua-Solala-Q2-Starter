use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub last_group: u8,
    pub group_count: u8
}