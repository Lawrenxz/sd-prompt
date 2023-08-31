use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub last_prompt: u8,
    pub prompt_count: u8,
}

#[account]
#[derive(Default)]
pub struct PromptAccount {
    pub authority: Pubkey,
    pub idx: u8,
    pub content: String,
    pub image_url: String,
}
