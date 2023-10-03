// Import necessary libraries and modules
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BalanceResponse, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{DebtResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{USER_BALANCE, USER_BORROW};

const DENOM: &str = "uoaksec";

// Instantiate function called when the contract is initialized
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Check if the contract instantiation has the required amount of funds
    if info.funds.len() != 1
        || info.funds[0].denom != DENOM
        || info.funds[0].amount != Uint128::from(1000_u64)
    {
        return Err(ContractError::InvalidInstantiation {});
    }

    // Return a success response with an attribute indicating the action
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// Execute function called when interacting with the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Match the provided ExecuteMsg to the corresponding function
    match msg {
        ExecuteMsg::Deposit {} => try_deposit(deps, info),
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, info, amount),
        ExecuteMsg::Borrow { amount } => try_borrow(deps, info, amount),
        ExecuteMsg::Repay {} => try_repay(deps, info),
    }
}

// Deposit function to add funds to the contract
pub fn try_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate that the correct token denomination is being deposited
    if info.funds.len() != 1 || info.funds[0].denom != DENOM {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid deposit!",
        )));
    }

    // Update the user's balance in storage
    USER_BALANCE.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance
                .unwrap_or_default()
                .checked_add(info.funds[0].amount)?)
        },
    )?;

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_attribute("method", "deposit")
        .add_attribute("amount", info.funds[0].amount))
}

// Withdraw function to remove funds from the contract
pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Validate that the withdrawal amount is non-zero
    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid zero amount!",
        )));
    }

    // Calculate the available amount the user can withdraw
    let deposit_amount = USER_BALANCE.load(deps.storage, &info.sender)?;

    let borrowed_amount = USER_BORROW
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    let available_to_withdraw = get_available_withdraw_amount(deposit_amount, borrowed_amount);

    if amount > available_to_withdraw {
        return Err(ContractError::Std(StdError::generic_err(
            "Withdraw too much!",
        )));
    }

    // Decrease user balance in storage
    USER_BALANCE.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    });

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount))
}

// Borrow function to take out a loan
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Validate that the borrow amount is non-zero
    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid zero amount!",
        )));
    }

    // Calculate the available amount the user can borrow
    let deposit_amount = USER_BALANCE.load(deps.storage, &info.sender)?;
    let mut borrowed_amount = USER_BORROW
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    let available_to_borrow = get_available_borrow_amount(deposit_amount, borrowed_amount);

    if amount > available_to_borrow {
        return Err(ContractError::Std(StdError::generic_err(
            "Borrow too much!",
        )));
    }

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    });

    // Update the user's borrowed amount in storage
    borrowed_amount += amount;

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "borrow")
        .add_attribute("amount", amount))
}

// Helper function to calculate the available amount to borrow
fn get_available_borrow_amount(deposit_amount: Uint128, borrowed_amount: Uint128) -> Uint128 {
    // Maximum borrow amount is 50% of the total deposit
    (deposit_amount * Decimal::from_ratio(1_u64, 2_u64)) - borrowed_amount
}

// Helper function to calculate the available amount to withdraw
fn get_available_withdraw_amount(deposit_amount: Uint128, borrowed_amount: Uint128) -> Uint128 {
    deposit_amount - (borrowed_amount * Decimal::from_ratio(2_u64, 1_u64))
}

// Repay function to return borrowed funds
pub fn try_repay(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate that the correct amount of funds is sent
    if info.funds.len() != 1 || info.funds[0].denom != DENOM {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid repayment!",
        )));
    }

    // Update user borrow balance in storage
    USER_BORROW.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance
                .unwrap_or_default()
                .checked_sub(info.funds[0].amount)?)
        },
    )?;

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_attribute("method", "repayment")
        .add_attribute("amount", info.funds[0].amount))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::GetDebt { address } => to_binary(&query_debt(deps, address)?),
    }
}

/// Query balance for a specific user
fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let user_balance = USER_BALANCE.load(deps.storage, &deps.api.addr_validate(&address)?)?;
    Ok(BalanceResponse {
        amount: Coin {
            denom: DENOM.to_string(),
            amount: user_balance,
        },
    })
}

/// Query debt for a specific user
fn query_debt(deps: Deps, address: String) -> StdResult<DebtResponse> {
    let user_debt = USER_BORROW
        .may_load(deps.storage, &deps.api.addr_validate(&address)?)?
        .unwrap_or_default();
    Ok(DebtResponse { amount: user_debt })
}