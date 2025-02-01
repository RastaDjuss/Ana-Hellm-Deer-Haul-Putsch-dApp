use anchor_lang::prelude::*;
use num_derive::FromPrimitive; // Pour générer le trait FromPrimitive
use num_traits::FromPrimitive; // Pour utiliser le trait FromPrimitive
#[error_code]
#[derive(Eq, PartialEq)]
pub enum EscrowErrorCode {
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded,
    #[msg("Current SOL price is not above Escrow unlock price.")]
    SolPriceBelowUnlockPrice,
}