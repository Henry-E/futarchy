use anchor_lang::prelude::*;

#[error_code]
pub enum ArbBotError {
    #[msg("One of the balances decreased")]
    BalanceDecreased,
    #[msg("USDC didn't increase")]
    QuoteDidNotIncrease,
}
