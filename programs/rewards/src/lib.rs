use anchor_lang::prelude::*;
use rand::Rng;

declare_id!("AAv2GZesLSUJm2Q63prFfkztTYEgzXLWfTfBJygC2dW5");

#[program]
mod rewards {
    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let reward_state = &mut ctx.accounts.reward_state;
        reward_state.total_tasks = 0;
        reward_state.total_users = 0;
        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>, activity_type: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_accounts;
        let reward_state = &mut ctx.accounts.reward_state;
        let clock = Clock::get()?;

        // check if cooldown period is going on
        if let Some(last_time) = user_account.last_task_time {
            require!(
                clock.unix_timestamp - last_time >= 5,
                CustomeError::CooldownNotElapsed
            );
        }

        // RNG-based taks availability
        let mut rng = rand::thread_rng();
        let task_available: bool = rng.gen();
        // let task_available = rand::random::<bool>();
        require!(task_available, CustomeError::TaskUnavailable);

        let mut base_reward = match activity_type.as_str() {
            "Check-In" | "View Analytics" | "Vote in a Pool" | "Subscribe to a Smart Contract" => {
                10_000_000
            }
            "Cast a Vote"
            | "Send a Message"
            | "Refer a User"
            | "Complete a Tutorial on Solana Usage"
            | "Test a Beta Feature on a dApp"
            | "Review a Smart Contract’s Code" => 50_000_000,
            "Deploy a Sample Smart Contract"
            | "Stake SOL for at Least 7 Days"
            | "Mint and Transfer an NFT"
            | "Provide Liquidity to a Protocol"
            | "Run a Validator Node for 24 Hours"
            | "Contribute Code to an Open-Source Project" => 100_000_000,
            _ => return Err(CustomeError::InvalidActivity.into()),
        };

        // Adjust reward dynamically based on demand-supply
        if reward_state.total_tasks > reward_state.total_users {
            // increasing the reward by 20%
            base_reward = (base_reward as f64 * 1.2) as u64;
        } else if reward_state.total_users > reward_state.total_tasks {
            // decreasing the reward by 10%
            base_reward = (base_reward as f64 * 0.9) as u64;
        }

        // Implement anti-faming penalty
        if let Some(last_task) = &user_account.last_task {
            if last_task == &activity_type {
                user_account.repeated_task_count += 1;
            } else {
                user_account.repeated_task_count = 1;
            }

            if user_account.repeated_task_count >= 3 {
                base_reward = user_account.rewards[user_account.rewards.len() - 1].amount / 2;
                // base_reward /= 2;
            }
        }

        // Store reward history
        user_account.rewards.push(Reward {
            user: *ctx.accounts.signer.key,
            amount: base_reward,
            activity_type: activity_type.clone(),
            timestamp: clock.unix_timestamp,
        });

        user_account.last_task = Some(activity_type);
        user_account.last_task_time = Some(clock.unix_timestamp);

        // Transfer rewards to user
        let reward_instruction = system_instruction::transfer(
            &ctx.accounts.reward_state.to_account_info().key,
            &ctx.accounts.signer.to_account_info().key,
            base_reward,
        );

        invoke(
            &reward_instruction,
            &[
                ctx.accounts.reward_state.to_account_info(),
                ctx.accounts.signer.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8 + 8)]
    pub reward_state: Account<'info, RewardState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub user_accounts: Account<'info, UserAccount>,
    #[account(mut)]
    pub reward_state: Account<'info, RewardState>,
    pub signer: Signer<'info>,
}

#[account]
pub struct RewardState {
    pub total_users: u64,
    pub total_tasks: u64,
}
#[account]
pub struct UserAccount {
    pub rewards: Vec<Reward>,
    pub last_task: Option<String>,
    pub last_task_time: Option<i64>,
    pub repeated_task_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Reward {
    pub user: Pubkey,
    pub amount: u64,
    pub activity_type: String,
    pub timestamp: i64,
}

#[error_code]
pub enum CustomeError {
    #[msg("Cooldown perio has not elapsed yet")]
    CooldownNotElapsed,
    #[msg("Task is not currently available")]
    TaskUnavailable,
    #[msg("Invalid activity selected")]
    InvalidActivity,
}
