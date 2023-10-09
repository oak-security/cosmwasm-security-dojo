// Import necessary libraries and modules
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary,Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{TestResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::TEST;


// Instantiate function called when the contract is initialized
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    TEST.save(deps.storage, &"instantiate".to_string());

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
        ExecuteMsg::DoSomething {} => try_something(deps, info),
    }
}

// Deposit function to add funds to the contract
pub fn try_something(deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {

    TEST.save(deps.storage, &"something".to_string())?;
    return Err( ContractError::Unauthorized{ } );
    TEST.save(deps.storage, &"something else".to_string())?;

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_attribute("method", "smthg"))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTest { } => to_binary(&query_test(deps,)?),
    }
}

/// Query balance for a specific user
fn query_test(deps: Deps) -> StdResult<TestResponse> {
    let resp = TEST.load(deps.storage)?;
    Ok(TestResponse {
        resp 
    })
}

