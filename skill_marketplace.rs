use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint, Token};

declare_id!("AIXSkillMarketplace111111111111111111111111");

#[program]
pub mod aix_skill_marketplace {
    use super::*;

    pub fn create_skill_token(
        ctx: Context<CreateSkillToken>,
        skill_name: String,
        skill_description: String,
        complexity: u8,
    ) -> Result<()> {
        // Create a new skill token
        let skill_token = &mut ctx.accounts.skill_token;
        skill_token.owner = ctx.accounts.authority.key();
        skill_token.skill_name = skill_name;
        skill_token.skill_description = skill_description;
        skill_token.complexity = complexity;
        skill_token.reputation_score = 0;
        
        Ok(())
    }

    pub fn list_skill(
        ctx: Context<ListSkill>,
        price: u64,
    ) -> Result<()> {
        // List a skill for sale in the marketplace
        let listing = &mut ctx.accounts.skill_listing;
        listing.skill_token = ctx.accounts.skill_token.key();
        listing.seller = ctx.accounts.authority.key();
        listing.price = price;
        listing.is_active = true;
        
        Ok(())
    }

    pub fn purchase_skill(
        ctx: Context<PurchaseSkill>,
    ) -> Result<()> {
        // Handle skill purchase and transfer
        let transfer_instruction = token::transfer(
            ctx.accounts.into(),
            ctx.accounts.skill_listing.price,
        )?;

        // Update skill ownership
        Ok(())
    }

    pub fn update_reputation(
        ctx: Context<UpdateReputation>,
        score_change: i16,
    ) -> Result<()> {
        // Adjust skill token's reputation score
        let skill_token = &mut ctx.accounts.skill_token;
        skill_token.reputation_score = skill_token.reputation_score.checked_add(score_change)
            .ok_or(ErrorCode::ReputationOverflow)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSkillToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init, 
        payer = authority, 
        space = 8 + 32 + 32 + 32 + 1 + 2
    )]
    pub skill_token: Account<'info, SkillToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListSkill<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub skill_token: Account<'info, SkillToken>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8 + 1
    )]
    pub skill_listing: Account<'info, SkillListing>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseSkill<'info> {
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub skill_listing: Account<'info, SkillListing>,
    #[account(mut)]
    pub skill_token: Account<'info, SkillToken>,
}

#[derive(Accounts)]
pub struct UpdateReputation<'info> {
    #[account(mut)]
    pub skill_token: Account<'info, SkillToken>,
    pub authority: Signer<'info>,
}

#[account]
pub struct SkillToken {
    pub owner: Pubkey,
    pub skill_name: String,
    pub skill_description: String,
    pub complexity: u8,
    pub reputation_score: i16,
}

#[account]
pub struct SkillListing {
    pub skill_token: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub is_active: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Reputation score calculation would cause an overflow")]
    ReputationOverflow,
}