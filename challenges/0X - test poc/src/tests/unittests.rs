use super::*;
use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary};
use crate::ContractError;
use crate::contract::{execute, instantiate};
use crate::contract::query;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, TestResponse};


#[test]
fn deposit_success() {
    let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    let msg = InstantiateMsg {};
    let info = mock_info("creator", &coins(1000, DENOM.to_string()));
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // user able to deposit funds
    let info = mock_info("alice", &coins(100, DENOM));
    let msg = ExecuteMsg::DoSomething {};
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    // verify deposit succeeded
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetTest {},
    )
    .unwrap();

    let value: TestResponse = from_binary(&res).unwrap();
    assert_eq!(value.resp, "something");
}
