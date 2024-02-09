use anchor_lang::prelude::*;

declare_id!("BiHSo6R3efRTpzJKWNSjiUvG6ucS3kiQeZzm4NZ37WdP");

#[program]
pub mod hello_world_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
