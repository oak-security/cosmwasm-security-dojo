# Exploit Explanation

In this document, we'll walk you through the security vulnerability that can be exploited in this CosmWasm smart contract due to permissionless access to the `update_config` function and improper resetting of fees.

Before reading further, we encourage you to try finding the vulnerability in the code itself. Come back here once you have found the issue or want to see the solution.

The contract allows users to deposit, withdraw, borrow, and repay funds. However, it contains a vulnerability that enables anyone to call the `update_config` function and update the owner of the contract, potentially allowing them to steal all the fees accumulated in the contract.

## A Quick Look at the Issue

There are two issues in the contract:

1. The `try_update_config` function allows updating the contract owner without checking if the sender of the transaction is the current owner.

```rust
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
```

2. The `try_withdraw_fees` function does not reset the accumulated fees to zero after a successful withdrawal.

```rust
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
```

## How to Fix the Issue

### Fixing the `try_update_config` function

To fix the first issue, we need to check if the sender of the transaction is the current owner before allowing the update of the contract owner.

Here's the improved `try_update_config` function with the fix applied:

```rust
pub fn try_update_config(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Check if the sender is the current owner
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    config.owner = deps.api.addr_validate(&new_owner)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "update_config")
        .add_attribute("new_owner", new_owner))
}
```

### Fixing the `try_withdraw_fees` function

To fix the second issue, we need to reset the accumulated fees to zero after a successful withdrawal.

Here's the improved `try_withdraw_fees` function with the fix applied:

```rust
pub fn try_withdraw_fees(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    } else if config.total_fee.is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let fees_to_withdraw = config.total_fee;

    // Reset the accumulated fees to zero
    config.total_fee = Uint128::zero();
    CONFIG.save(deps.storage, &config)?;

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
```

By adding the line `config.total_fee = Uint128::zero();` and updating the config in storage with `CONFIG.save(deps.storage, &config)?;`, we ensure that the accumulated fees are reset to zero after a successful withdrawal.

## Conclusion

In this document, we have discussed two security vulnerabilities in the given CosmWasm smart contract:

1. The `try_update_config` function allowed anyone to change the contract owner, which should have been restricted to the current owner.
2. The `try_withdraw_fees` function did not reset the accumulated fees to zero after a successful withdrawal, making it possible to withdraw the fees multiple times.

When combined, these vulnerabilities create a scenario where a malicious user can update the contract owner without permission and then exploit the fee withdrawal issue to steal funds from the contract.

Now you've learned about this simple vulnerability and know how to fix it! Keep exploring, and don't hesitate to dive deeper into CosmWasm smart contract security. The more you learn, the more you'll be prepared to develop secure smart contracts and spot common pitfalls. Happy learning! 🚀
