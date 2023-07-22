use anchor_lang::prelude::*;

declare_id!("5Jp7hr8taYJYMcaaVirQTvEJwrAeFzWbwnyBwVNZq7um");

#[program]
pub mod helloworld {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("hello world,from solana smart contract");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
