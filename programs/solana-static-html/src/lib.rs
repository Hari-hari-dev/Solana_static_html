use anchor_lang::prelude::*;

declare_id!("XAe6yJkYSa2V5PgMK9a77BARucgzA9mFAqbuEHQJBeD");

#[program]
pub mod solana_static_html {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
