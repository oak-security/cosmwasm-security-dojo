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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    #[should_panic(expected = "Invalid instantiation")]
    fn invalid_init() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(0, DENOM.to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn deposit_success() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, DENOM.to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // user able to deposit uosmo
        let info = mock_info("alice", &coins(100, DENOM));
        let msg = ExecuteMsg::Deposit {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // verify deposit succeeded
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetBalance {
                address: "alice".to_string(),
            },
        )
        .unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Uint128::from(100_u64), value.amount.amount);
    }

    #[test]
    #[should_panic(expected = "Invalid deposit!")]
    fn deposit_failure() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, DENOM.to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // other funds such as uluna with not be recorded
        let info = mock_info("bob", &coins(10, "uluna".to_string()));
        let msg = ExecuteMsg::Deposit {};
        let _err = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn borrow_success() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, DENOM.to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // alice deposits 1000 funds
        let info = mock_info("alice", &coins(1_000, DENOM));
        let msg = ExecuteMsg::Deposit {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // alice able borrow 500 funds (50% of 1000)
        let empty_fund: Vec<Coin> = vec![];
        let info = mock_info("alice", &empty_fund);
        let msg = ExecuteMsg::Borrow {
            amount: Uint128::from(500_u64),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    #[should_panic(expected = "Borrow too much!")]
    fn borrow_fail() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, DENOM.to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // bob deposits 1000 funds
        let info = mock_info("bob", &coins(1_000, DENOM));
        let msg = ExecuteMsg::Deposit {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // bob cannot borrow more than 50% of deposited funds
        let empty_fund: Vec<Coin> = vec![];
        let info = mock_info("bob", &empty_fund);
        let msg = ExecuteMsg::Borrow {
            amount: Uint128::from(501_u64),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn exploit() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, DENOM.to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // hacker deposits 1000 funds
        let info = mock_info("hacker", &coins(1_000, DENOM));
        let msg = ExecuteMsg::Deposit {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let empty_fund: Vec<Coin> = vec![];
        let info = mock_info("hacker", &empty_fund);
        let msg = ExecuteMsg::Borrow {
            amount: Uint128::from(500_u64),
        };

        // hacker able to repeatly borrow 500 funds because USER_BORROW is not saved into storage
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
