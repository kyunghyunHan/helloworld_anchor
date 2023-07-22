use anchor_lang::prelude::*;

declare_id!("EyPYzW5RV3Fn643HKaA52boiUysPbwKbXfPZpBRUsStu");

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
