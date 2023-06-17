use anchor_lang::prelude::*;

#[account]
pub struct SavingsGroup {
    pub owner: Pubkey,
    pub name_of_group: String,
    pub duration_of_contribution: u8,
    pub contribution_amount: u128,
    pub group_description: String,
    pub group_terms: String,
    pub max_number: u8,
    pub total_money: u64,
    pub active: bool,
    pub account: Pubkey
}

#[account]
pub struct SavingsGroupList {
    pub savings_groups: Vec<SavingsGroup>
}