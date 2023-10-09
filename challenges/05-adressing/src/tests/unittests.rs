use super::*;
use crate::contract::query;
use crate::contract::{execute, instantiate};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, UserResponse};
use crate::ContractError;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary};
use cosmwasm_std:: Uint128;

#[test]
fn deposit_withdraw() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {initial_deny: vec![]};
    let info = mock_info("creator", &coins(0, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Deposit funds
    let info = mock_info("alice", &coins(100, DENOM));
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Verify deposit succeeded
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetUserData {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: UserResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::from(100_u64), value.amount);

    // Withdraw funds
    let info = mock_info("alice", &[]);
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::from(100_u64),
        destination: Some("rcpt".to_string()),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

}


#[test]
fn denylisting() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {initial_deny: vec![]};
    let info = mock_info("creator", &coins(0, DENOM.to_string()));
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Deposit funds
    let info = mock_info("alice", &coins(100, DENOM));
    let msg = ExecuteMsg::Deposit {};
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Denylist rcpt
    let info = mock_info("creator", &[]);
    let msg = ExecuteMsg::AddToDenylist { address: "rcpt".to_string() };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Verify deposit succeeded
    {
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetUserData {
                address: "alice".to_string(),
            },
        )
        .unwrap();
        let value: UserResponse = from_binary(&res).unwrap();
        assert_eq!(Uint128::from(100_u64), value.amount);
    }

    // Failed withdraw of funds
    let info = mock_info("alice", &[]);
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::from(100_u64),
        destination: Some("rcpt".to_string()),
    };
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(res, ContractError::Denylisted {  });

    // Verify deposit succeeded   
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetUserData {
            address: "alice".to_string(),
        },
    )
    .unwrap();
    let value: UserResponse = from_binary(&res).unwrap();
    assert_eq!(Uint128::from(100_u64), value.amount); //@todo although returning an error in 77 the withdraw takes effect??
    

    // Allow-list rcpt
    let info = mock_info("creator", &[]);
    let msg = ExecuteMsg::RemoveFromDenylist { address: "rcpt".to_string() };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Successful withdraw of funds
    let info = mock_info("alice", &[]);
    let msg = ExecuteMsg::Withdraw {
        amount: Uint128::from(100_u64),
        destination: Some("rcpt".to_string()),
    };
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();
}

