use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub stats: UserStats,
    pub name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserStats {
    pub level: u8,
    pub xp: u64,
}
