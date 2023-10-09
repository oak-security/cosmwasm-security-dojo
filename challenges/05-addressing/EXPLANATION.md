# Exploit Explanation 

In this document, we'll walk you through the security vulnerability that can be exploited in this CosmWasm smart contract.

Before reading further, we encourage you to try finding the vulnerability in the code itself. Come back here once you have found the issue or want to see the solution.

The contract allows users to deposit withdraw funds controlled by a denylist. However, it contains a bug that allows users to bypass the denying mechanism. The reason is that a destination address is not validated nor normalized. Don't worry if you're new to this, we'll guide you step by step through the issue.

## A Quick Look at the Issue

The issue lies in the `try_withdraw` function of the contract. This function sends user funds to an address or their choosing. But it misses a crucial step: validating and normalizing the destination address.

```rust
pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    destination: Option<String>,
) -> Result<Response, ContractError> {
    // ... (omitted for brevity)

    // Assign recipient
    let mut recipient = info.sender.to_string();
    if let Some(destination) = destination {
        check_denylist(&deps, &destination)?;
        recipient = destination;
    }

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient,
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount: amount_to_withdraw,
        }],
    });

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount))
}
```

## How to Fix the Issue

Fear not! Fixing the issue is quite simple. You need to check the user provided address. Here's the improved `try_withdraw` function with the fix applied:

```rust
pub fn try_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    destination: Option<String>,
) -> Result<Response, ContractError> {
    // ... (omitted for brevity)

    // Assign recipient
    let mut recipient = info.sender.to_string();
    if let Some(destination) = destination {
        check_denylist(&deps, &deps.api.addr_validate(&destination)?)?;
        recipient = destination;
    }

    // Send tokens to the user
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient,
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount: amount_to_withdraw,
        }],
    });

    // Return a success response with attributes indicating the method and amount
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "withdraw")
        .add_attribute("amount", amount))
}
```

By adding the line `deps.api.addr_validate(&destination)?`, we ensure that the provided address is both validated and checked for normalization. This prevents the user from bypassing the denylist mechanism.

Now you've learned about this simple vulnerability and know how to fix it! Keep exploring, and don't hesitate to dive deeper into CosmWasm smart contract security. The more you learn, the more you'll be prepared to develop secure smart contracts and spot common pitfalls. Happy learning! 🚀