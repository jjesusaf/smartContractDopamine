use anchor_lang::prelude::*;

declare_id!("8HmGVibjBETPYJToiQCxZEzJyBQocgEYDgk7bU7Nkuru");

#[program]
pub mod dopamine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
