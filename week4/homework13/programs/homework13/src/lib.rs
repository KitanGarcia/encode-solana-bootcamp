use anchor_lang::prelude::*;

declare_id!("GovAH137uoY4ifyfTNQjesKNJKo55tMF435dBfwZ22DG");

#[program]
pub mod homework13 {
    use super::*;

    pub fn initialize(ctx: Context<Create>) -> Result<()> {
        let balance_struct: &mut Account<BalanceStruct> = &mut ctx.accounts.balance_struct; 
        balance_struct.balance = 100;
        msg!("{}", balance_struct.balance);
        Ok(())
    }
}

// Contexts
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = authority, space = 8 + 180)]
    pub balance_struct: Account<'info, BalanceStruct>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Account
#[account]
pub struct BalanceStruct {    
    pub authority: Pubkey, 
    pub balance: u64,
}
