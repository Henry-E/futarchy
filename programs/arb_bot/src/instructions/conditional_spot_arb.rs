use anchor_lang::prelude::*;
use anchor_spl::token::*;
use meteora_pools::cpi::{accounts::Swap as MeteoraSwap, swap as meteora_swap};
use conditional_vault::cpi::{accounts::InteractWithVault, mint_conditional_tokens, merge_conditional_tokens_for_underlying_tokens};
use amm::cpi::{accounts::Swap as MetaSwap, swap as meta_swap};
use amm::instructions::swap::SwapArgs;
use amm::state::SwapType;

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
    #[account(mut, token::mint = user_base.mint)]
    pub a_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = user_quote.mint)]
    pub b_vault: Account<'info, TokenAccount>,
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
    pub admin_token_a_fee: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin_token_b_fee: UncheckedAccount<'info>,

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
    pub conditional_vault_program: UncheckedAccount<'info>,
    pub meta_amm_program: UncheckedAccount<'info>,
    pub token_program: UncheckedAccount<'info>,
    pub associated_token_program: UncheckedAccount<'info>,
    pub system_program: UncheckedAccount<'info>
}

impl ConditionalSpotArb<'_> {
    pub fn execute(ctx: Context<ConditionalSpotArb>) -> Result<()> {
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
            admin_token_a_fee,
            admin_token_b_fee,

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
            conditional_vault_program,
            meta_amm_program,
            token_program,
            associated_token_program,
            system_program,
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
            let usdc_in_amount: u64 = 1_000_000;
            let meta_minimum_out_amount: u64 = 1_000;
            let old_meta_balance = user_base.amount;

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
                admin_token_fee: admin_token_b_fee.to_account_info(),
                user: user.to_account_info(),
                vault_program: meteora_vault_program.to_account_info(),
                token_program: token_program.to_account_info(),
            };
            let meteora_ctx = CpiContext::new(meteora_pools_program.to_account_info(), meteora_accounts);
            meteora_swap(meteora_ctx, usdc_in_amount, meta_minimum_out_amount)?;

            user_base.reload()?;
            let conditional_meta_amount_to_mint = user_base.amount.checked_sub(old_meta_balance).unwrap();

            let mint_conditional_base_accounts = InteractWithVault{
                vault: conditional_base_vault.to_account_info(),
                conditional_on_finalize_token_mint: conditional_base_pass_mint.to_account_info(),
                conditional_on_revert_token_mint: conditional_base_fail_mint.to_account_info(),
                vault_underlying_token_account: vault_base_underlying.to_account_info(),
                authority: user.to_account_info(),
                user_conditional_on_finalize_token_account: user_base_pass.to_account_info(),
                user_conditional_on_revert_token_account: user_base_fail.to_account_info(),
                user_underlying_token_account: user_base.to_account_info(),
                token_program: token_program.to_account_info(),
            };
            let mint_conditional_base_ctx = CpiContext::new(conditional_vault_program.to_account_info(), mint_conditional_base_accounts);
            mint_conditional_tokens(mint_conditional_base_ctx, conditional_meta_amount_to_mint)?;

            let old_quote_pass_balance = user_quote_pass.amount;
            let old_quote_fail_balance = user_quote_fail.amount;

            // Now swap in both conditional markets
            for (conditional_amm, conditional_base_mint, conditional_quote_mint, 
                user_ata_base, user_ata_quote, vault_ata_base, vault_ata_quote) in [
                (amm_pass,conditional_base_pass_mint.to_account_info(), conditional_quote_pass_mint.to_account_info(),
                user_base_pass.to_account_info(), user_quote_pass.to_account_info(), vault_base_pass, vault_quote_pass
                 ),
                (amm_fail, conditional_base_fail_mint.to_account_info(), conditional_quote_fail_mint.to_account_info(),
                user_base_fail.to_account_info(), user_quote_fail.to_account_info(), vault_base_fail, vault_quote_fail
                )
            ] {
                let swap_accounts = MetaSwap {
                    user: user.to_account_info(),
                    amm: conditional_amm.to_account_info(),
                    base_mint: conditional_base_mint,
                    quote_mint: conditional_quote_mint,
                    // user_ata_base: user_ata_base.to_account_info(),
                    user_ata_base,
                    // user_ata_quote: user_ata_quote.to_account_info(),
                    user_ata_quote,
                    vault_ata_base: vault_ata_base.to_account_info(),
                    vault_ata_quote: vault_ata_quote.to_account_info(),
                    associated_token_program: associated_token_program.to_account_info(),
                    token_program: token_program.to_account_info(),
                    system_program: system_program.to_account_info(),
                };
                let swap_ctx = CpiContext::new(meta_amm_program.to_account_info(), swap_accounts);
                let swap_args = SwapArgs {
                    input_amount: conditional_meta_amount_to_mint,
                    output_amount_min: 1_000,
                    swap_type: SwapType::Sell
                };
                meta_swap(swap_ctx, swap_args)?;
            }

            // Swapped both tokens, now to merge to get the underlying
            user_quote_pass.reload()?;
            user_quote_fail.reload()?;
            
            let swapped_quote_pass_amount = user_quote_pass.amount.checked_sub(old_quote_pass_balance).unwrap();
            let swapped_quote_fail_amount = user_quote_fail.amount.checked_sub(old_quote_fail_balance).unwrap();
            let min_swapped_amount = swapped_quote_pass_amount.min(swapped_quote_fail_amount);

            let merge_conditional_quote_accounts = InteractWithVault{
                vault: conditional_quote_vault.to_account_info(),
                conditional_on_finalize_token_mint: conditional_quote_pass_mint.to_account_info(),
                conditional_on_revert_token_mint: conditional_quote_fail_mint.to_account_info(),
                vault_underlying_token_account: vault_quote_underlying.to_account_info(),
                authority: user.to_account_info(),
                user_conditional_on_finalize_token_account: user_quote_pass.to_account_info(),
                user_conditional_on_revert_token_account: user_quote_fail.to_account_info(),
                user_underlying_token_account: user_quote.to_account_info(),
                token_program: token_program.to_account_info(),
            };
            let mint_conditional_quote_ctx = CpiContext::new(conditional_vault_program.to_account_info(), merge_conditional_quote_accounts);
            merge_conditional_tokens_for_underlying_tokens(mint_conditional_quote_ctx, min_swapped_amount)?;


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
        ConditionalSpotArb::check_balance_changes(initial_balances, final_balances)?;

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

    fn check_balance_changes(initial: Vec<u64>, final_: Vec<u64>) -> Result<()> {
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
