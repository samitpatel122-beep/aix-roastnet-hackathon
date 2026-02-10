use anchor_lang::prelude::*;

declare_id!("RoastNetViralGrowth11111111111111111111111111");

#[program]
pub mod roast_net_viral {
    use super::*;

    pub fn create_roast_challenge(
        ctx: Context<CreateChallenge>,
        project_details: String,
        roast_intensity: u8,
    ) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        challenge.initiator = ctx.accounts.initiator.key();
        challenge.project_details = project_details;
        challenge.roast_intensity = roast_intensity;
        challenge.challenge_token = generate_unique_token(
            &challenge.initiator, 
            &project_details
        );
        challenge.validation_status = ValidationStatus::Pending;
        
        Ok(())
    }

    pub fn validate_roast_challenge(
        ctx: Context<ValidateChallenge>,
        proof_link: String,
        platform: SocialPlatform,
    ) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        let validator = &ctx.accounts.validator;

        // Validate social media proof
        require!(
            validate_social_proof(&proof_link, platform),
            RoastNetError::InvalidValidationProof
        );

        challenge.validator = validator.key();
        challenge.validation_status = ValidationStatus::Verified;
        challenge.validation_platform = platform;
        
        // Award reputation points
        award_reputation_points(validator.key(), challenge.roast_intensity);

        Ok(())
    }
}

fn generate_unique_token(initiator: &Pubkey, project_details: &str) -> [u8; 32] {
    // Generate a deterministic, unique token
    let mut token = [0u8; 32];
    let hash_input = format!("{}{}", initiator, project_details);
    let hash = anchor_lang::solana_program::keccak::hash(hash_input.as_bytes());
    token.copy_from_slice(hash.as_ref());
    token
}

fn validate_social_proof(proof_link: &str, platform: SocialPlatform) -> bool {
    // Placeholder for social media validation logic
    // Would integrate with platform APIs in production
    match platform {
        SocialPlatform::Twitter => proof_link.contains("twitter.com"),
        SocialPlatform::LinkedIn => proof_link.contains("linkedin.com"),
        SocialPlatform::Reddit => proof_link.contains("reddit.com"),
        SocialPlatform::PersonalBlog => true, // More complex validation needed
    }
}

fn award_reputation_points(validator: Pubkey, roast_intensity: u8) -> Result<()> {
    // Award reputation based on roast intensity
    // Placeholder for reputation tracking logic
    Ok(())
}

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    #[account(
        init,
        payer = initiator,
        space = 8 + 32 + 32 + 1 + 32 + 1
    )]
    pub challenge: Account<'info, RoastChallenge>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidateChallenge<'info> {
    #[account(mut)]
    pub challenge: Account<'info, RoastChallenge>,
    pub validator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RoastChallenge {
    pub initiator: Pubkey,
    pub validator: Pubkey,
    pub project_details: String,
    pub roast_intensity: u8,
    pub challenge_token: [u8; 32],
    pub validation_status: ValidationStatus,
    pub validation_platform: SocialPlatform,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum ValidationStatus {
    Pending,
    Verified,
    Rejected,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum SocialPlatform {
    Twitter,
    LinkedIn,
    Reddit,
    PersonalBlog,
}

#[error_code]
pub enum RoastNetError {
    #[msg("Invalid validation proof")]
    InvalidValidationProof,
}