use anchor_lang::prelude::*;

pub mod constant;

pub mod states;
use crate::{constant::*, states::*};

declare_id!("CZP97Bh6aJG4LqYs44Yo8oLGJU9EgGffMi2zcp4ruuzD");

#[program]
pub mod sd_prompt {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // Initialize user profile with default data
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_prompt = 0;
        user_profile.prompt_count = 0;

        Ok(())
    }

    pub fn add_prompt(ctx: Context<AddPrompt>, content: String, image_url: String) -> Result<()> {
        let prompt_account = &mut ctx.accounts.prompt_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Fill contents with argument
        prompt_account.authority = ctx.accounts.authority.key();
        prompt_account.idx = user_profile.last_prompt;
        prompt_account.content = content;
        prompt_account.image_url = image_url;

        // Increase prompt idx for PDA
        user_profile.last_prompt = user_profile.last_prompt.checked_add(1).unwrap();

        // Increase total prompt count
        user_profile.prompt_count = user_profile.prompt_count.checked_add(1).unwrap();

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddPrompt<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [PROMPT_TAG, authority.key().as_ref(), &[user_profile.last_prompt as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<PromptAccount>() + 100 + 100,
    )]
    pub prompt_account: Box<Account<'info, PromptAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
