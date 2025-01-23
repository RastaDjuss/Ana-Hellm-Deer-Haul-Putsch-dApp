use anchor_lang::prelude::*;

declare_id!("CPek9mXxyHpiimTc6NrU6h1WTBun1p93442YmP5fewtM");

pub mod governanceprogramlibrary {
    use super::*;

    pub fn close(_ctx: Context<Closegovernanceprogramlibrary>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.governanceprogramlibrary.count = ctx.accounts.governanceprogramlibrary.count.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.governanceprogramlibrary.count = ctx.accounts.governanceprogramlibrary.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn initialize(_ctx: Context<InitializeAgovernanceprogramlibrary>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.governanceprogramlibrary.count = value.clone();
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initializegovernanceprogramlibrary<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  init,
  space = 8 + governanceprogramlibrary::INIT_SPACE,
  payer = payer
    )]
    pub governanceprogramlibrary: Account<'info, governanceprogramlibrary>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Closegovernanceprogramlibrary<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  init,
  space = 8 + governanceprogramlibrary::INIT_SPACE,
  payer = payer
    )]
    pub governanceprogramlibrary: Account<'info, governanceprogramlibrary>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub governanceprogramlibrary: Account<'info, governanceprogramlibrary>,
}
#[account]
#[derive(InitSpace)]
pub struct governanceprogramlibrary {
    count: u8,
}
