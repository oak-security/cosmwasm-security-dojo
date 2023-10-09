// Import necessary libraries and modules
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
};
use cw_utils::must_pay;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, UserResponse};
use crate::state::{UserDetails, DENYLIST, OWNER, USERS};

const DENOM: &str = "uoaksec";

// Instantiate function called when the contract is initialized
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender)?;

    DENYLIST.save(deps.storage, &msg.initial_deny)?;

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
        ExecuteMsg::Withdraw {
            amount,
            destination,
        } => try_withdraw(deps, info, amount, destination),
        ExecuteMsg::AddToDenylist { address } => try_add_deny(deps, info, address),
        ExecuteMsg::RemoveFromDenylist { address } => try_remove_deny(deps, info, address),
        ExecuteMsg::DistributeRewards {} => try_distribute(deps, info),
    }
}

// Deposit function to add funds to the contract
pub fn try_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    check_denylist(&deps, &info.sender.to_string())?;

    // Validate that the correct token denomination is being deposited
    let amount = must_pay(&info, DENOM).unwrap();

    // Update the user's balance in storage
    USERS.update(
        deps.storage,
        &info.sender,
        |balance: Option<UserDetails>| -> StdResult<_> {
            let balance = balance.unwrap_or_default();
            Ok(UserDetails {
                deposit: balance.deposit.checked_add(amount)?,
                rewards: balance.rewards,
            })
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
    destination: Option<String>,
) -> Result<Response, ContractError> {
    check_denylist(&deps, &info.sender.to_string())?;

    // Validate that the withdrawal amount is non-zero
    if amount.is_zero() {
        return Err(ContractError::ZeroAmount {});
    }

    // Calculate the available amount the user can withdraw
    let mut user = USERS.load(deps.storage, &info.sender)?;

    if amount > user.deposit {
        return Err(ContractError::WithdrawExceeded {});
    }

    // Decrease user balance in storage and add any rewards
    user.deposit = user
        .deposit
        .checked_sub(amount)
        .map_err(StdError::overflow)?;
    let amount_to_withdraw = amount + user.rewards;
    user.rewards = Uint128::zero();
    USERS.save(deps.storage, &info.sender, &user)?;

    println!("1");

    // Assign recipient
    let mut recipient = info.sender.to_string();
    if let Some(destination) = destination {
        check_denylist(&deps, &destination)?;

        let denylist = DENYLIST.load(deps.storage)?;
        if denylist.contains(&destination) {
            println!("Err Deny in line");
            return Err(ContractError::Denylisted {});
        }

        recipient = destination;
    }

    println!("2");

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient,
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount: amount_to_withdraw,
        }],
    });

    println!("3");

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount))
}

// Privileged. Add address to the denylist
pub fn try_add_deny(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    if info.sender != OWNER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let address = deps.api.addr_validate(&address)?;

    check_denylist(&deps, &address.to_string())?;

    let mut list = DENYLIST.load(deps.storage).unwrap();

    list.push(address.to_string());

    DENYLIST.save(deps.storage, &list)?;

    // Return a success response with attributes indicating the method and address
    Ok(Response::new()
        .add_attribute("method", "AddToDenylist")
        .add_attribute("address", address.to_string()))
}

//  Privileged. Remove address from the denylist
pub fn try_remove_deny(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    if info.sender != OWNER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    let address = deps.api.addr_validate(&address)?.to_string();

    // Validate that the address is actually part of the current list
    let mut current_denylist = DENYLIST.load(deps.storage)?;
    if !current_denylist.contains(&address) {
        return Err(ContractError::Std(StdError::generic_err(
            "Address not in denylist!",
        )));
    }

    current_denylist.retain(|addr| addr != &address);

    DENYLIST.save(deps.storage, &current_denylist)?;

    // Return a success response with attributes indicating the method and address
    Ok(Response::new()
        .add_attribute("method", "RemoveFromDenylist")
        .add_attribute("address", address))
}

//  Privileged. Distribute rewards.
pub fn try_distribute(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    if info.sender != OWNER.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    /*
       Redacted for simplicity purposes
    */

    // Return a success response with attributes indicating the method
    Ok(Response::new().add_attribute("method", "DistributeRewards"))
}

// Helper function to check if an address is denylisted
pub fn check_denylist(deps: &DepsMut, address: &String) -> Result<(), ContractError> {
    let denylist = DENYLIST.load(deps.storage)?;

    if denylist.contains(address) {
        println!("Err Deny in fn");
        return Err(ContractError::Denylisted {});
    }

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUserData { address } => to_binary(&query_user(deps, address)?),
        QueryMsg::GetOwner {} => to_binary(&OWNER.load(deps.storage)?),
        QueryMsg::GetIsDenied { address } => to_binary(&query_denylist(deps, address)?),
    }
}

/// Query balance for a specific user
fn query_user(deps: Deps, address: String) -> StdResult<UserResponse> {
    let user = USERS.load(deps.storage, &deps.api.addr_validate(&address)?)?;
    Ok(UserResponse {
        amount: user.deposit,
        rewards: user.rewards,
    })
}

/// Query if an address is denylisted
fn query_denylist(deps: Deps, address: String) -> StdResult<bool> {
    let denylist = DENYLIST.load(deps.storage)?;

    if denylist.contains(&address) {
        return Ok(true);
    }

    Ok(false)
}
