use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, MintTo};
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("92CPjhz5G19stRScGBnvK6VeSXioTTcSHACJGW72cWGB");

#[allow(dead_code)]
const MIN_STAKING_PERIOD: i64 = 1 * 1000;
#[allow(dead_code)]
const APY: f64 = 20.0;
#[program]
pub mod mango {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let pool_info = &mut ctx.accounts.pool_info;
        let start = Clock::get().unwrap().unix_timestamp;

        pool_info.admin = ctx.accounts.admin.key();
        pool_info.start_slot = start as u64;
        pool_info.end_slot = i64::MAX as u64;
        pool_info.token = ctx.accounts.staking_token.key();
        pool_info.tvl = 0;
        pool_info.minted_reward = 0;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        let user_info = &mut ctx.accounts.user_info;
        let pool_info = &mut ctx.accounts.pool_info;
        let clock = Clock::get()?;

        // update user reward
        let mut reward = clock
            .slot
            .checked_sub(user_info.deposit_slot)
            .expect("Overflow error. Clock slot is filled");
        reward = reward
            .checked_mul(user_info.amount)
            .expect("Overflow error: reward too high");

        // update user info
        user_info.amount += amount;
        user_info.deposit_slot = clock.slot;
        user_info.reward = reward;

        // update tvl
        pool_info.tvl += amount;

        // transfer native sol to admin
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user_staking_wallet.to_account_info().key(),
            &ctx.accounts.admin_staking_wallet.to_account_info().key(),
            amount,
        );
        let tx = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user_staking_wallet.to_account_info(),
                ctx.accounts.admin_staking_wallet.to_account_info(),
            ],
        );
        assert!(
            tx.is_ok(),
            "{:?}",
            msg!("Staking Failed: could not transfer SOL")
        );

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> ProgramResult {
        let user_info = &mut ctx.accounts.user_info;
        let clock = Clock::get()?;

        // update user reward
        let mut reward = clock
            .slot
            .checked_sub(user_info.deposit_slot)
            .expect("Overflow error. Clock slot is filled");
        reward = reward
            .checked_mul(user_info.amount)
            .expect("Overflow error: reward too high");

        // update pool info
        let pool_info = &mut ctx.accounts.pool_info;
        pool_info.tvl -= user_info.amount;

        // update user info
        user_info.amount = 0;
        user_info.deposit_slot = 0;
        user_info.reward = reward;

        // transfer SOL token to staker
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.admin_staking_wallet.to_account_info().key(),
            &ctx.accounts.user_staking_wallet.to_account_info().key(),
            user_info.amount,
        );
        let tx = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.admin_staking_wallet.to_account_info(),
                ctx.accounts.user_staking_wallet.to_account_info(),
            ],
        );
        assert!(
            tx.is_ok(),
            "{:?}",
            msg!("UnStaking Failed: could not transfer SOL")
        );

        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> ProgramResult {
        let user_info = &mut ctx.accounts.user_info;
        let pool_info = &mut ctx.accounts.pool_info;
        let clock = Clock::get()?;

        // update user reward
        let mut reward = clock
            .slot
            .checked_sub(user_info.deposit_slot)
            .expect("Overflow error. Clock slot is filled");
        reward = reward
            .checked_mul(user_info.amount)
            .expect("Overflow error: reward too high");

        // update user info
        user_info.reward = 0;

        // update pool info
        pool_info.minted_reward += reward;

        // transfer 5% of reward token to staker referral
        let referee_reward = (reward * 5).checked_div(100).unwrap();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.staking_token.to_account_info(),
            to: ctx.accounts.referral.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, reward - referee_reward)?;

        // transfer 95% of reward token to staker
        let cpi_accounts = MintTo {
            mint: ctx.accounts.staking_token.to_account_info(),
            to: ctx.accounts.user_staking_wallet.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, reward - referee_reward)?;

        Ok(())
    }

    pub fn add_referral(ctx: Context<Stake>, referral: Pubkey) -> ProgramResult {
        let user_info = &mut ctx.accounts.user_info;
        user_info.referral = Some(referral);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = admin, 
        space = 1024 + PoolInfo::LEN, 
        seeds = [b"auction", admin.key().as_ref(), payer.key().as_ref(), 8u64.to_le_bytes().as_ref()], bump)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub staking_token: Account<'info, Mint>,
    #[account(mut)]
    pub admin_staking_wallet: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(init, payer = user, space = 1024 + UserInfo::LEN)]
    pub user_info: Account<'info, UserInfo>,
    #[account(init, payer = user, space = 1024 + PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub user_staking_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin_staking_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token: Account<'info, Mint>,
    pub token_program: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,
    #[account(mut)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub user_staking_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin_staking_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token: Account<'info, Mint>,
    pub token_program: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    /// CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub referral: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,
    #[account(mut)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub user_staking_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin_staking_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token: Account<'info, Mint>,
    pub token_program: Account<'info, TokenAccount>,
}
#[account]
pub struct PoolInfo {
    pub admin: Pubkey,
    pub start_slot: u64,
    pub end_slot: u64,
    pub token: Pubkey,
    pub tvl: u64,
    pub minted_reward: u64,
}
#[account]
pub struct UserInfo {
    pub amount: u64,
    pub reward: u64,
    pub deposit_slot: u64,
    pub referral: Option<Pubkey>,
}
impl UserInfo {
    pub const LEN: usize = 8 + 8 + 8;
}
impl PoolInfo {
    pub const LEN: usize = 32 + 8 + 8 + 32;
}
