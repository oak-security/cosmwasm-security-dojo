#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BalanceResponse, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{DebtResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{USER_BALANCE, USER_BORROW};

const DENOM: &str = "uosmo";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // admin must provide 1000 uosmo when instantiating contract
    if info.funds.len() != 1
        || info.funds[0].denom != DENOM
        || info.funds[0].amount != Uint128::from(1000_u64)
    {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid instantiation",
        )));
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => try_deposit(deps, info),
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, info, amount),
        ExecuteMsg::Borrow { amount } => try_borrow(deps, info, amount),
        ExecuteMsg::Repay {} => try_repay(deps, info),
    }
}

/// deposit funds entry point
pub fn try_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // validate uosmo sent
    if info.funds.len() != 1 || info.funds[0].denom != DENOM {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid deposit!",
        )));
    }

    // update user balance
    USER_BALANCE.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance
                .unwrap_or_default()
                .checked_add(info.funds[0].amount)?)
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "deposit")
        .add_attribute("amount", info.funds[0].amount))
}

/// withdraw funds entry point
pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid zero amount!",
        )));
    }

    // get user deposit amount
    let deposit_amount = USER_BALANCE.load(deps.storage, &info.sender)?;

    // get user borrow amount
    let borrowed_amount = USER_BORROW
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    let available_to_withdraw = get_available_withdraw_amount(deposit_amount, borrowed_amount);

    if amount > available_to_withdraw {
        return Err(ContractError::Std(StdError::generic_err(
            "Withdraw too much!",
        )));
    }

    // decrease user balance
    USER_BALANCE.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;

    // send uosmo to user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount))
}

/// borrow funds entry point
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid zero amount!",
        )));
    }

    // get user deposit amount
    let deposit_amount = USER_BALANCE.load(deps.storage, &info.sender)?;

    // get user borrow amount
    let mut borrowed_amount = USER_BORROW
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    let available_to_borrow = get_available_borrow_amount(deposit_amount, borrowed_amount);

    if amount > available_to_borrow {
        return Err(ContractError::Std(StdError::generic_err(
            "Borrow too much!",
        )));
    }

    // send uosmo to user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    });

    // update user borrow amount
    borrowed_amount += amount;

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "borrow")
        .add_attribute("amount", amount))
}

fn get_available_borrow_amount(deposit_amount: Uint128, borrowed_amount: Uint128) -> Uint128 {
    // max borrow amount = 50% of total deposit
    (deposit_amount * Decimal::from_ratio(1_u64, 2_u64)) - borrowed_amount
}

fn get_available_withdraw_amount(deposit_amount: Uint128, borrowed_amount: Uint128) -> Uint128 {
    deposit_amount - (borrowed_amount * Decimal::from_ratio(2_u64, 1_u64))
}

pub fn try_repay(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // validate uosmo sent
    if info.funds.len() != 1 || info.funds[0].denom != DENOM {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid repayment!",
        )));
    }

    // update user balance
    USER_BORROW.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance
                .unwrap_or_default()
                .checked_sub(info.funds[0].amount)?)
        },
    )?;

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

fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let user_balance = USER_BALANCE.load(deps.storage, &deps.api.addr_validate(&address)?)?;
    Ok(BalanceResponse {
        amount: Coin {
            denom: DENOM.to_string(),
            amount: user_balance,
        },
    })
}

fn query_debt(deps: Deps, address: String) -> StdResult<DebtResponse> {
    let user_debt = USER_BORROW
        .may_load(deps.storage, &deps.api.addr_validate(&address)?)?
        .unwrap_or_default();
    Ok(DebtResponse { amount: user_debt })
}
