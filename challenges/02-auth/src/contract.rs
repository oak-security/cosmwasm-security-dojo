#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BalanceResponse, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{DebtResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, UserDetails, CONFIG, USERS};

const DENOM: &str = "uoaksec";

// Instantiate function called when the contract is initialized
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
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

    // Save owner address
    let config = Config {
        owner: info.sender,
        total_fee: Uint128::zero(),
    };

    CONFIG.save(deps.storage, &config).unwrap();

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
        ExecuteMsg::UpdateConfig { new_owner } => try_update_config(deps, info, new_owner),
        ExecuteMsg::WithdrawFees {} => try_withdraw_fees(deps, info),
    }
}

/// Deposit function to add funds to the contract
pub fn try_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate that the correct token denomination is being deposited
    if info.funds.len() != 1 || info.funds[0].denom != DENOM {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid deposit!",
        )));
    }

    let amount = info.funds[0].amount;

    // Update the user's balance in storage
    let mut user = USERS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    user.deposit += amount;

    USERS.save(deps.storage, &info.sender, &user)?;

    Ok(Response::new()
        .add_attribute("method", "deposit")
        .add_attribute("amount", info.funds[0].amount))
}

/// Withdraw function to remove funds from the contract
pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Validate that the withdrawal amount is non-zero
    if amount.is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    // Calculate the available amount the user can withdraw
    let mut user = USERS.load(deps.storage, &info.sender)?;

    let UserDetails { deposit, borrowed } = user;

    let available_to_withdraw = get_available_withdraw_amount(deposit, borrowed);

    if amount > available_to_withdraw {
        return Err(ContractError::Std(StdError::generic_err(
            "Withdraw too much!",
        )));
    }

    user.deposit -= amount;

    USERS.save(deps.storage, &info.sender, &user)?;

    // collect platform fee
    let fee = amount * Decimal::percent(5);

    CONFIG.update(deps.storage, |mut config| -> Result<Config, StdError> {
        config.total_fee += fee;
        Ok(config)
    })?;

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount: amount - fee,
        }],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount - fee))
}

/// Borrow function to take out a loan
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Validate that the borrow amount is non-zero
    if amount.is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    // Calculate the available amount the user can borrow
    let mut user = USERS.load(deps.storage, &info.sender)?;

    let UserDetails { deposit, borrowed } = user;

    let available_to_borrow = get_available_borrow_amount(deposit, borrowed);

    if amount > available_to_borrow {
        return Err(ContractError::Std(StdError::generic_err(
            "Borrow too much!",
        )));
    }

    // Update the user's borrowed amount in storage
    user.borrowed += amount;

    USERS.save(deps.storage, &info.sender, &user)?;

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "borrow")
        .add_attribute("amount", amount))
}

/// Updates contract owner
pub fn try_update_config(
    deps: DepsMut,
    _info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    config.owner = deps.api.addr_validate(&new_owner)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "update_config")
        .add_attribute("new_owner", new_owner))
}

/// Withdraw platform fees
pub fn try_withdraw_fees(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    } else if config.total_fee.is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let fees_to_withdraw = config.total_fee;

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: config.owner.to_string(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount: fees_to_withdraw,
        }],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw_fees")
        .add_attribute("fees_to_withdraw", fees_to_withdraw))
}

/// Helper function to calculate the available amount to borrow
fn get_available_borrow_amount(deposit_amount: Uint128, borrowed_amount: Uint128) -> Uint128 {
    // Maximum borrow amount is 50% of the total deposit
    (deposit_amount * Decimal::from_ratio(1_u64, 2_u64)) - borrowed_amount
}

/// Helper function to calculate the available amount to withdraw
fn get_available_withdraw_amount(deposit_amount: Uint128, borrowed_amount: Uint128) -> Uint128 {
    deposit_amount - (borrowed_amount * Decimal::from_ratio(2_u64, 1_u64))
}

/// Repay function to return borrowed funds
pub fn try_repay(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Validate that the correct amount of funds is sent
    if info.funds.len() != 1 || info.funds[0].denom != DENOM {
        return Err(ContractError::Std(StdError::generic_err(
            "Invalid repayment!",
        )));
    }

    let amount = info.funds[0].amount;

    let mut user = USERS.load(deps.storage, &info.sender)?;

    user.borrowed -= amount;

    USERS.save(deps.storage, &info.sender, &user)?;

    Ok(Response::new()
        .add_attribute("method", "repayment")
        .add_attribute("amount", amount))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::GetDebt { address } => to_binary(&query_debt(deps, address)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

/// Query balance for a specific user
fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let user = USERS
        .load(deps.storage, &deps.api.addr_validate(&address)?)
        .unwrap_or_default();
    Ok(BalanceResponse {
        amount: Coin {
            denom: DENOM.to_string(),
            amount: user.deposit,
        },
    })
}

/// Query debt for a specific user
fn query_debt(deps: Deps, address: String) -> StdResult<DebtResponse> {
    let user = USERS
        .may_load(deps.storage, &deps.api.addr_validate(&address)?)?
        .unwrap_or_default();
    Ok(DebtResponse {
        amount: user.borrowed,
    })
}

/// Query contract config
fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}
