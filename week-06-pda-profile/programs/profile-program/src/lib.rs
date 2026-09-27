use anchor_lang::prelude::*;

declare_id!("63LK3qMWZVJ8tseoUyhS9v5miZK9U829ExyQcGHJZQyw");

#[program]
pub mod profile_program {
    use super::*;

    pub fn create_profile(
        ctx: Context<CreateProfile>,
        name: String,
        bio: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        profile.owner = ctx.accounts.user.key();
        profile.name = name;
        profile.bio = bio;

        msg!("Profile created!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Profile::INIT_SPACE,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub owner: Pubkey,

    #[max_len(32)]
    pub name: String,

    #[max_len(100)]
    pub bio: String,
}