#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod votingapp {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        _poll_id: u64,
        poll_start: u64,
        poll_end: u64,
        name: String,
        description: String) -> Result<()> {
        ctx.accounts.poll.name = name;
        ctx.accounts.poll.poll_start = poll_start;
        ctx.accounts.poll.poll_end = poll_end;
        ctx.accounts.poll.description = description;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        _poll_id: u64,
        candidate_name: String,
    ) -> Result<()> {
        ctx.accounts.candidate.candidate_name = candidate_name;
        ctx.accounts.candidate.candidate_votes += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
      let candidate_account = &mut ctx.accounts.candidate_account;
      let current_time = Clock::get()?.unix_timestamp;

      if current_time > (ctx.accounts.poll_account.poll_end as i64) {
          return Err(ErrorCode::VotingEnded.into());
      }

      if current_time <= (ctx.accounts.poll_account.poll_start as i64) {
          return Err(ErrorCode::VotingNotStarted.into());
      }

      candidate_account.candidate_votes += 1;

      Ok(())
  }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE, 
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate_name: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Candidate::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
        bump,
    )]

    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, Candidate>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    #[max_len(32)]
    pub name: String,
    #[max_len(280)]
    pub description: String,
    pub poll_id: u64,
    pub poll_start: u64,
    pub poll_end: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}