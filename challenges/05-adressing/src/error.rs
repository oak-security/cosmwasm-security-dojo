use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    //@todo
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid instantiation.")]
    InvalidInstantiation {},

    #[error("Address is part of the denylist.")]
    Denylisted {},    

    #[error("Tried to withdraw in more than owned.")]
    WithdrawExceeded {},  

    #[error("Can't withdraw zero.")]
    ZeroAmount {},  

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
