# Exploit Explanation

Welcome to the CosmWasm exploit explanation! In this document, we'll walk you through a simple exploit found in a CosmWasm smart contract. The contract allows users to deposit, borrow, and repay funds. However, it contains a bug that allows users to repeatedly borrow funds without updating their debt in the contract's storage. Don't worry if you're new to this; we'll guide you step by step.

## A Quick Look at the Issue

The issue lies in the `try_borrow` function of the contract. It calculates the amount a user can borrow and sends the funds to the user. But it misses a crucial step: updating the user's borrow amount in the contract's storage.

```rust
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // ... (omitted for brevity)

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
```

## How to Fix the Issue

Fear not! Fixing the issue is quite simple. You need to update the user's borrow amount in the contract's storage. Here's the improved `try_borrow` function with the fix applied:

```rust
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // ... (omitted for brevity)

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

    // Save the updated borrow amount to storage
    USER_BORROW.save(deps.storage, &info.sender.to_string(), &borrowed_amount)?;

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "borrow")
        .add_attribute("amount", amount))
}
```

By adding the line `USER_BORROW.save(deps.storage, &info.sender.to_string(), &borrowed_amount)?;`, we ensure that the user's borrow amount is properly updated in the contract's storage. This prevents the user from repeatedly borrowing funds without updating their debt.

Now you've learned about this simple exploit and how to fix it! Keep exploring, and don't hesitate to dive deeper into CosmWasm smart contract security. The more you learn, the more you'll be prepared to develop secure smart contracts and spot common pitfalls. Happy learning! 🚀