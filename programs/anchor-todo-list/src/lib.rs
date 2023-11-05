use anchor_lang::prelude::*;

declare_id!("6aWv5V6DNuDGCZciWpL2RHj315z5vL5DhaTyV3Pj4hB");

#[program]
pub mod anchor_todo_list {
    use super::*;

    pub fn adding_task(ctx: Context<AddingTask>, title: String) -> Result<()> {
        if title.chars().count() > 400 {
            return Err(ErrorCode::TitleTooLong.into());
        }
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        task.author = *author.key;
        task.completed = false;
        task.created_at = clock.unix_timestamp;
        task.updated_at = clock.unix_timestamp;
        task.title = title;
        Ok(())
    }

    pub fn updating_task(ctx: Context<UpdatingTask>, completed: bool) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        task.author = *author.key;
        task.completed = completed;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn deleting_task(ctx: Context<DeletingTask>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();
        task.author = *author.key;
        task.completed = true;
        task.updated_at = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AddingTask<'info> {
    #[account(init, payer = author, space = Task::LEN)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletingTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Task {
    pub author: Pubkey,
    pub title: String,
    pub completed: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TITLE_LENGTH: usize = 4 + 4 * 400; // 400 chars
const BOOL_LENGTH: usize = 1;
const TIMESTAMP_LENGTH: usize = 8;

impl Task {
    pub const LEN: usize = DISCRIMINATOR
        + PUBLIC_KEY_LENGTH
        + TITLE_LENGTH
        + BOOL_LENGTH
        + TIMESTAMP_LENGTH
        + TIMESTAMP_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The title is too long")]
    TitleTooLong,
}
