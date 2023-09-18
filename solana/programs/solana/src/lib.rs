use anchor_lang::prelude::*;

declare_id!("kMmjwkAfsWhh2m6mqkEgn3i4BdrTZFMow8UeTvLq25K");

#[program]
pub mod solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count -= 1;
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space=8 + 8)]
    pub counter_account: Account<'info, CounterAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    pub count: u8,
}
