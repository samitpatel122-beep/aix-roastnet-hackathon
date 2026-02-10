use anchor_lang::prelude::*;

declare_id!("RoastNetPlatform11111111111111111111111111");

#[program]
pub mod roast_net {
    use super::*;

    pub fn initiate_roast_challenge(
        ctx: Context<InitiateChallenge>,
        challenge_text: String,
    ) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        challenge.challenger = ctx.accounts.challenger.key();
        challenge.challenge_text = challenge_text;
        challenge.status = ChallengeStatus::Active;
        
        Ok(())
    }

    pub fn submit_roast(
        ctx: Context<SubmitRoast>,
        roast_content: String,
    ) -> Result<()> {
        let roast = &mut ctx.accounts.roast;
        roast.roaster = ctx.accounts.roaster.key();
        roast.content = roast_content;
        roast.wit_score = calculate_wit_score(&roast_content);
        
        Ok(())
    }

    pub fn vote_on_roast(
        ctx: Context<VoteOnRoast>,
        vote_weight: u8,
    ) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        vote.voter = ctx.accounts.voter.key();
        vote.weight = vote_weight;
        
        Ok(())
    }
}

fn calculate_wit_score(roast: &str) -> u16 {
    // Placeholder wit scoring algorithm
    // Would be replaced with sophisticated NLP analysis
    let base_score = roast.len() as u16;
    let complexity_bonus = roast.split_whitespace().count() as u16;
    base_score + complexity_bonus
}

#[derive(Accounts)]
pub struct InitiateChallenge<'info> {
    #[account(mut)]
    pub challenger: Signer<'info>,
    #[account(
        init,
        payer = challenger,
        space = 8 + 32 + 32 + 1
    )]
    pub challenge: Account<'info, RoastChallenge>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitRoast<'info> {
    #[account(mut)]
    pub roaster: Signer<'info>,
    #[account(
        init,
        payer = roaster,
        space = 8 + 32 + 32 + 2
    )]
    pub roast: Account<'info, Roast>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnRoast<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        init,
        payer = voter,
        space = 8 + 32 + 1
    )]
    pub vote: Account<'info, RoastVote>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RoastChallenge {
    pub challenger: Pubkey,
    pub challenge_text: String,
    pub status: ChallengeStatus,
}

#[account]
pub struct Roast {
    pub roaster: Pubkey,
    pub content: String,
    pub wit_score: u16,
}

#[account]
pub struct RoastVote {
    pub voter: Pubkey,
    pub weight: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ChallengeStatus {
    Active,
    Resolved,
    Expired,
}