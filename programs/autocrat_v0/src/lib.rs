use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
// use conditional_vault::program::ConditionalVault;
use clob::state::order_book::OrderBook;
use conditional_vault::ConditionalVault as ConditionalVaultAccount;

// by default, the pass price needs to be 20% higher than the fail price
pub const DEFAULT_PASS_THRESHOLD_BPS: u16 = 2000;
// pub const WSOL: Pubkey = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

pub use wsol::ID as WSOL;
mod wsol {
    use super::*;
    declare_id!("So11111111111111111111111111111111111111112");
}

declare_id!("5QBbGKFSoL1hS4s5dsCBdNRVnJcMuHXFwhooKk2ar25S");

#[account]
pub struct DAO {
    pub token: Pubkey,
    // the percentage, in basis points, the pass price needs to be above the
    // fail price in order for the proposal to pass
    pub pass_threshold_bps: u16,
}

#[account]
pub struct Proposal {
    pub did_execute: bool,
    pub instructions: Vec<ProposalInstruction>,
    pub accounts: Vec<ProposalAccount>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProposalInstruction {
    pub program_id: Pubkey,
    // Accounts to pass to the target program, stored as
    // indexes into the `proposal.accounts` vector.
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProposalAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[program]
pub mod autocrat_v0 {
    use super::*;

    pub fn initialize_dao(ctx: Context<InitializeDAO>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;

        dao.token = ctx.accounts.token.key();
        dao.pass_threshold_bps = DEFAULT_PASS_THRESHOLD_BPS;

        Ok(())
    }

    pub fn initialize_proposal(
        ctx: Context<InitializeProposal>,
        instructions: Vec<ProposalInstruction>,
        accts: Vec<ProposalAccount>,
    ) -> Result<()> {
        let pass_market = ctx.accounts.pass_market.load()?;
        let fail_market = ctx.accounts.fail_market.load()?;

        require!(
            pass_market.base == ctx.accounts.base_pass_vault.conditional_token_mint,
            AutocratError::InvalidMarket
        );
        require!(
            pass_market.quote == ctx.accounts.quote_pass_vault.conditional_token_mint,
            AutocratError::InvalidMarket
        );

        require!(
            fail_market.base == ctx.accounts.base_fail_vault.conditional_token_mint,
            AutocratError::InvalidMarket
        );
        require!(
            fail_market.quote == ctx.accounts.quote_fail_vault.conditional_token_mint,
            AutocratError::InvalidMarket
        );

        let proposal = &mut ctx.accounts.proposal;

        let (quote_pass_settlement_authority, _) =
            Pubkey::find_program_address(&[proposal.key().as_ref(), b"quote_pass"], &self::ID);
        let (base_pass_settlement_authority, _) =
            Pubkey::find_program_address(&[proposal.key().as_ref(), b"base_pass"], &self::ID);
        let (quote_fail_settlement_authority, _) =
            Pubkey::find_program_address(&[proposal.key().as_ref(), b"quote_fail"], &self::ID);
        let (base_fail_settlement_authority, _) =
            Pubkey::find_program_address(&[proposal.key().as_ref(), b"base_fail"], &self::ID);

        require!(
            ctx.accounts.quote_pass_vault.settlement_authority == quote_pass_settlement_authority,
            AutocratError::InvalidSettlementAuthority
        );
        require!(
            ctx.accounts.base_pass_vault.settlement_authority == base_pass_settlement_authority,
            AutocratError::InvalidSettlementAuthority
        );
        require!(
            ctx.accounts.quote_fail_vault.settlement_authority == quote_fail_settlement_authority,
            AutocratError::InvalidSettlementAuthority
        );
        require!(
            ctx.accounts.base_fail_vault.settlement_authority == base_fail_settlement_authority,
            AutocratError::InvalidSettlementAuthority
        );

        proposal.did_execute = false;
        proposal.instructions = instructions;
        proposal.accounts = accts;

        Ok(())
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        // TODO: verify that 10 days worth of slot time has passed
        // TODO: verify that pass price TWAP is `threshold_percent` over the fail price
        // TODO: execute proposal
        Ok(())
    }

    pub fn set_pass_threshold_bps(ctx: Context<Auth>, pass_threshold_bps: u16) -> Result<()> {
        let dao = &mut ctx.accounts.dao;

        dao.pass_threshold_bps = pass_threshold_bps;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDAO<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 2,
        seeds = [b"WWCACOTMICMIBMHAFTTWYGHMB"], // abbreviation of the last two sentences of the Declaration of Independence of Cyberspace
        bump
    )]
    pub dao: Account<'info, DAO>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mint::decimals = 9)]
    pub token: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    #[account(zero)]
    pub proposal: Account<'info, Proposal>,
    pub dao: Account<'info, DAO>,
    #[account(
        constraint = quote_pass_vault.underlying_token_mint == dao.token,
    )]
    pub quote_pass_vault: Account<'info, ConditionalVaultAccount>,
    #[account(
        constraint = quote_fail_vault.underlying_token_mint == dao.token,
    )]
    pub quote_fail_vault: Account<'info, ConditionalVaultAccount>,
    #[account(
        constraint = base_pass_vault.underlying_token_mint == WSOL,
    )]
    pub base_pass_vault: Account<'info, ConditionalVaultAccount>,
    #[account(
        constraint = base_fail_vault.underlying_token_mint == WSOL,
    )]
    pub base_fail_vault: Account<'info, ConditionalVaultAccount>,
    pub pass_market: AccountLoader<'info, OrderBook>,
    pub fail_market: AccountLoader<'info, OrderBook>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(
        // signer @ ErrorCode::UnauthorizedFunctionCall,
        mut
    )]
    pub dao: Account<'info, DAO>,
}

impl From<&ProposalAccount> for AccountMeta {
    fn from(acc: &ProposalAccount) -> Self {
        Self {
            pubkey: acc.pubkey,
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        }
    }
}

#[error_code]
pub enum AutocratError {
    #[msg(
        "Either the `pass_market` or the `fail_market`'s tokens doesn't match the vaults supplied"
    )]
    InvalidMarket,
    #[msg("One of the vaults has an invalid `settlement_authority`")]
    InvalidSettlementAuthority,
}