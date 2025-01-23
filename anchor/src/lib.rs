use anchor_lang::prelude::*;

// This is your program's public key and it will update automatically when you build the project.
declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod journal {
    use super::*;
}
#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub message: String,
}
#[program]
mod journal {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        msg!("Journal Entry Created");
        msg!("Title: {}", title);
        msg!("Message: {}", message);

        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = ctx.accounts.owner.key();
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    #[account(
        init_if_needed,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + JournalEntryState::INIT_SPACE
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[program]
mod journal {
    use super::*;

    pub fn update_journal_entry(
        ctx: Context<UpdateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        msg!("Journal Entry Updated");
        msg!("Title: {}", title);
        msg!("Message: {}", message);

        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = message;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + title.len() + 4 + message.len(),
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[program]
mod journal {
    use super::*;
    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, title: String) -> Result<()> {
        msg!("Journal entry titled {} deleted", title);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}