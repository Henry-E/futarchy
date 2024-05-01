use anchor_lang::prelude::*;
use anchor_spl::token::*;
use meteora_pools::cpi::{accounts::Swap as MeteoraSwap, swap as meteora_swap};

use crate::error::ArbBotError;

#[derive(Accounts)]
pub struct ConditionalSpotArb<'info> {
    pub user: Signer<'info>,

    /// User token accounts
    #[account(mut)]
    pub user_quote: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_base: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_quote_pass: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_quote_fail: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_base_pass: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_base_fail: Account<'info, TokenAccount>,

    /// Meteora Swap
    #[account(mut)]
    pub meteora_pool: UncheckedAccount<'info>,
    #[account(mut)]
    pub a_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub b_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub a_token_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub b_token_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub a_vault_lp_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub b_vault_lp_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub a_vault_lp: UncheckedAccount<'info>,
    #[account(mut)]
    pub b_vault_lp: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin_token_fee: UncheckedAccount<'info>,

    /// AMM Swap
    #[account(mut)]
    pub amm_pass: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_base_pass: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_quote_pass: UncheckedAccount<'info>,

    #[account(mut)]
    pub amm_fail: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_base_fail: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_quote_fail: UncheckedAccount<'info>,

    /// Conditional Tokens
    #[account(mut)]
    pub conditional_quote_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub conditional_quote_pass_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub conditional_quote_fail_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_quote_underlying: UncheckedAccount<'info>,

    #[account(mut)]
    pub conditional_base_vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub conditional_base_pass_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub conditional_base_fail_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault_base_underlying: UncheckedAccount<'info>,

    /// Programs needed
    pub meteora_pools_program: UncheckedAccount<'info>,
    pub meteora_vault_program: UncheckedAccount<'info>,
    pub conditional_program: UncheckedAccount<'info>,
    pub amm_program: UncheckedAccount<'info>,
    pub token_program: UncheckedAccount<'info>,
}

impl ConditionalSpotArb<'_> {
    pub fn execute(&self, ctx: Context<ConditionalSpotArb>) -> Result<()> {
           // Destructure the account struct
           let ConditionalSpotArb {
            user,

            user_quote,
            user_base,
            user_quote_pass,
            user_quote_fail,
            user_base_pass,
            user_base_fail,

            meteora_pool,
            a_vault,
            b_vault,
            a_token_vault,
            b_token_vault,
            a_vault_lp_mint,
            b_vault_lp_mint,
            a_vault_lp,
            b_vault_lp,
            admin_token_fee,

            amm_pass,
            vault_base_pass,
            vault_quote_pass,
            amm_fail,
            vault_base_fail,
            vault_quote_fail,

            conditional_quote_vault,
            conditional_quote_pass_mint,
            conditional_quote_fail_mint,
            vault_quote_underlying,

            conditional_base_vault,
            conditional_base_pass_mint,
            conditional_base_fail_mint,
            vault_base_underlying,

            meteora_pools_program,
            meteora_vault_program,
            conditional_program,
            amm_program,
            token_program,
        } = ctx.accounts;

        // Get initial balances
        let initial_balances = vec![
            user_quote.amount,
            user_base.amount,
            user_quote_pass.amount,
            user_quote_fail.amount,
            user_base_pass.amount,
            user_base_fail.amount,
        ];

        // If statement
            // both rates above spot arb
        // else
            // both rates below spot arb
        if true {
            // pass
            let usdc_amount: u64 = 1_000_000;

            // TODO: add an if else statement to assign which accounts are which based on the 
            // a and b thing
            let meteora_accounts = MeteoraSwap {
                pool: meteora_pool.to_account_info(),
                user_source_token: user_quote.to_account_info(),
                user_destination_token: user_base.to_account_info(),
                a_vault: a_vault.to_account_info(),
                b_vault: b_vault.to_account_info(),
                a_token_vault: a_token_vault.to_account_info(),
                b_token_vault: b_token_vault.to_account_info(),
                a_vault_lp_mint: a_vault_lp_mint.to_account_info(),
                b_vault_lp_mint: b_vault_lp_mint.to_account_info(),
                a_vault_lp: a_vault_lp.to_account_info(),
                b_vault_lp: b_vault_lp.to_account_info(),
                admin_token_fee: admin_token_fee.to_account_info(),
                user: user.to_account_info(),
                vault_program: meteora_vault_program.to_account_info(),
                token_program: token_program.to_account_info(),
            };
            
        } else {
            // pass
        }

        // Reload accounts
        user_quote.reload()?;
        user_base.reload()?;
        user_quote_pass.reload()?;
        user_quote_fail.reload()?;
        user_base_pass.reload()?;
        user_base_fail.reload()?;

        // Get final balances
        let final_balances = vec![
            user_quote.amount,
            user_base.amount,
            user_quote_pass.amount,
            user_quote_fail.amount,
            user_base_pass.amount,
            user_base_fail.amount,
        ];

        // Check balance changes
        self.check_balance_changes(initial_balances, final_balances)?;

        Ok(())
    }

    fn check_arbitrage_opportunity(&self, _ctx: Context<Self>) -> Result<()> {
        Ok(())
    }

    fn get_balances(&self, ctx: Context<Self>) -> Vec<u64> {
        vec![
            ctx.accounts.user_quote.amount,
            ctx.accounts.user_base.amount,
            ctx.accounts.user_quote_pass.amount,
            ctx.accounts.user_quote_fail.amount,
            ctx.accounts.user_base_pass.amount,
            ctx.accounts.user_base_fail.amount,
        ]
    }

    fn reload_accounts(&self, ctx: Context<Self>) -> Result<()> {
        ctx.accounts.user_quote.reload()?;
        ctx.accounts.user_base.reload()?;
        ctx.accounts.user_quote_pass.reload()?;
        ctx.accounts.user_quote_fail.reload()?;
        ctx.accounts.user_base_pass.reload()?;
        ctx.accounts.user_base_fail.reload()?;
        Ok(())
    }

    fn check_balance_changes(&self, initial: Vec<u64>, final_: Vec<u64>) -> Result<()> {
        for (i, (init, fin)) in initial.iter().zip(final_.iter()).enumerate() {
            msg!(
                "Balance at index {} started at {} and ended at {}",
                i,
                init,
                fin
            );
            require_gte!(fin, init, ArbBotError::BalanceDecreased);
        }

        // Specifically check that user_quote has increased
        require_gt!(final_[0], initial[0], ArbBotError::QuoteDidNotIncrease);

        Ok(())
    }
}
