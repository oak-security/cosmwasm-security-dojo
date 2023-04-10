use super::*;
use crate::contract::{execute, instantiate, query};
use crate::msg::{DebtResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Config;
use crate::ContractError;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, BalanceResponse, Coin, Decimal, StdError, Uint128};

#[test]
fn invalid_init() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(0, DENOM.to_string()));
    let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, ContractError::InvalidInstantiation {});
}

#[test]
fn deposit_success() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // user able to deposit uosmo
    let info = mock_info("alice", &coins(100, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

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
fn deposit_failure() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // other funds such as uluna with not be recorded
    let info = mock_info("bob", &coins(10, "uluna".to_string()));
    let msg = ExecuteMsg::Deposit {};
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(
        res,
        ContractError::Std(StdError::generic_err("Invalid deposit!"))
    );
}

#[test]
fn borrow_success() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice deposits 1000 funds
    let info = mock_info("alice", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice able borrow 500 funds (50% of 1000)
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(500_u64),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
}

#[test]
fn borrow_fail() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // bob deposits 1000 funds
    let info = mock_info("bob", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // bob cannot borrow more than 50% of deposited funds
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("bob", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(501_u64),
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(
        res,
        ContractError::Std(StdError::generic_err("Borrow too much!"))
    );
}

#[test]
fn query_works() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetBalance {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::new(0), value.amount.amount);

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetDebt {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: DebtResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::new(0), value.amount);

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let value: Config = from_binary(&res).unwrap();
    assert_eq!(value.owner, "creator".to_string());
    assert_eq!(value.total_fee, Uint128::zero());
}

#[test]
fn repay_works() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice deposits 1000 funds
    let info = mock_info("alice", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice able borrow 500 funds (50% of 1000)
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Borrow {
        amount: Uint128::from(500_u64),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice repays her debt
    let info = mock_info("alice", &coins(500, DENOM));
    let msg = ExecuteMsg::Repay {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // all debt is repaid
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetDebt {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: DebtResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::new(0), value.amount);
}

#[test]
fn withdraw_with_fees() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice deposits 1000 funds
    let info = mock_info("alice", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice withdraw all her funds
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(1000),
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice only gets back 95% of her original funds
    assert_eq!(res.attributes.len(), 2);
    assert_eq!(res.attributes[0].value, "withdraw");
    assert_eq!(
        res.attributes[1].value,
        (Uint128::new(1000) * Decimal::percent(95)).to_string()
    );
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let value: Config = from_binary(&res).unwrap();
    assert_eq!(value.owner, "creator".to_string());

    let info = mock_info("creator", &[]);
    let msg = ExecuteMsg::UpdateConfig {
        new_owner: "alice".to_string(),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let value: Config = from_binary(&res).unwrap();
    assert_eq!(value.owner, "alice".to_string());
}

#[test]
fn owner_withdraw_fees() {
    let mut deps = mock_dependencies();

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice deposits 1000 funds
    let info = mock_info("alice", &coins(1_000, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // alice withdraw all her funds
    let empty_fund: Vec<Coin> = vec![];
    let info = mock_info("alice", &empty_fund);
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(1000),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // owner withdraw fees
    let info = mock_info("creator", &empty_fund);
    let msg = ExecuteMsg::WithdrawFees {};
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(res.attributes.len(), 2);
    assert_eq!(res.attributes[0].value, "withdraw_fees");
    assert_eq!(
        res.attributes[1].value,
        (Uint128::new(1000) * Decimal::percent(5)).to_string()
    );
}
