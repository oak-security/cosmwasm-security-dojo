use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid instantiation.")]
    InvalidInstantiation {},

    #[error("You can't operate with an amount of zero.")]
    ZeroAmount {},

    #[error("You can't withdraw more than you own.")]
    WithdrawExceeded {},

    #[error("You can't borrow more than {available}.")]
    BorrowExceeded { available:Uint128 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
