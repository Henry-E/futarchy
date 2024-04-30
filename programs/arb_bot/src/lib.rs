use anchor_lang::prelude::*;
use meteora_pools::cpi::{swap, accounts::Swap};

declare_id!("A43He8159Wx79j1tzZQqqfKTRNoj5xA3ScKRrgGo8Jb");

#[program]
pub mod arb_bot {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn execute_swap(ctx: Context<ExecuteSwap>, in_amount: u64, minimum_out_amount: u64) -> Result<()> {
        // Create a Swap context
        let cpi_accounts = Swap {
            pool: ctx.accounts.pool.to_account_info(),
            user_source_token: ctx.accounts.user_source_token.to_account_info(),
            user_destination_token: ctx.accounts.user_destination_token.to_account_info(),
            a_vault: ctx.accounts.a_vault.to_account_info(),
            b_vault: ctx.accounts.b_vault.to_account_info(),
            a_token_vault: ctx.accounts.a_token_vault.to_account_info(),
            b_token_vault: ctx.accounts.b_token_vault.to_account_info(),
            a_vault_lp_mint: ctx.accounts.a_vault_lp_mint.to_account_info(),
            b_vault_lp_mint: ctx.accounts.b_vault_lp_mint.to_account_info(),
            a_vault_lp: ctx.accounts.a_vault_lp.to_account_info(),
            b_vault_lp: ctx.accounts.b_vault_lp.to_account_info(),
            admin_token_fee: ctx.accounts.admin_token_fee.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            vault_program: ctx.accounts.vault_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.meteora_pools_program.to_account_info(), cpi_accounts);

        // Call the swap function with the appropriate amounts
        swap(cpi_ctx, in_amount, minimum_out_amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ExecuteSwap<'info> {
    /// CHECK: nothing
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub user_source_token: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub user_destination_token: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub a_vault: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub b_vault: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub a_token_vault: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub b_token_vault: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub a_vault_lp_mint: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub b_vault_lp_mint: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub a_vault_lp: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub b_vault_lp: UncheckedAccount<'info>,

    /// CHECK: nothing
    #[account(mut)]
    pub admin_token_fee: UncheckedAccount<'info>,

    /// CHECK: nothing
    pub user: Signer<'info>,

    /// CHECK: nothing
    pub vault_program: UncheckedAccount<'info>,

    /// CHECK: nothing
    pub token_program: UncheckedAccount<'info>,

    /// CHECK: nothing
    pub meteora_pools_program: UncheckedAccount<'info>,
}