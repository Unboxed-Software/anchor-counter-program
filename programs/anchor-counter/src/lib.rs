use anchor_lang::prelude::*;

declare_id!("9sMy4hnC9MML6mioESFZmzpntt3focqwUq1ymPgbMf64");

// Size of the anchor discrimitaror, needed for the space calculation.
const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialize the counter account with a count of 0.
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;

        // Log the current count.
        msg!("Counter account created. Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        // Load the current counter value.
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        // Increment the counter value and log it.
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR + Counter::INIT_SPACE,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}
