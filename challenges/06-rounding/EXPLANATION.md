# Exploit Explanation

In this document, we'll walk you through the security vulnerability that can be exploited in this CosmWasm smart contract.

Before reading further, we encourage you to try finding the vulnerability in the code itself. Come back here once you have found the issue or want to see the solution.

The contract allows users to deposit, borrow, and repay funds. However, it contains a bug that allows users to bypass the protocol fee charged upon borrowing. The reason is that fees are rounded floor wise by default. Don't worry if you're new to this, we'll guide you step by step through the issue.

## A Quick Look at the Issue

The issue lies in the `try_borrow` function of the contract. This function calculates the amount a user should get deducted as fee payment. But it fails to consider what happens with small enough borrows.

```rust
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // ... (omitted for brevity)

    // Update the user's borrowed amount in storage
    borrowed_amount += amount;

    // Save the updated borrow amount to storage
    USER_BORROW.save(deps.storage, &info.sender, &borrowed_amount)?;

    // Protocol Fee
    let fee = amount * Decimal::percent(FIXED_FEE_PERCENT);
    USER_BALANCE.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(fee)?)
        },
    )?;

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "borrow")
        .add_attribute("amount", amount))
}
```

## How to Fix the Issue

Fear not! Fixing the issue is quite simple. You need to explicitly set the result to be rounded ceil wise, therefore in favor of the protocol. Here's the improved `try_borrow` function with the fix applied:

```rust
pub fn try_borrow(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // ... (omitted for brevity)

    // Update the user's borrowed amount in storage
    borrowed_amount += amount;

    // Save the updated borrow amount to storage
    USER_BORROW.save(deps.storage, &info.sender, &borrowed_amount)?;

    // Protocol Fee
    let fee = amount.mul_ceil(Decimal::percent(FIXED_FEE_PERCENT));
    USER_BALANCE.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(fee)?)
        },
    )?;

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "borrow")
        .add_attribute("amount", amount))
}
```

By changing the affected line into `amount.mul_ceil(Decimal::percent(FIXED_FEE_PERCENT))`, we ensure that the result in rounded in favor of the protocol. This prevents the users from bypassing the fees with small operations.

Now you've learned about this simple vulnerability and know how to fix it! Keep exploring, and don't hesitate to dive deeper into CosmWasm smart contract security. The more you learn, the more you'll be prepared to develop secure smart contracts and spot common pitfalls. Happy learning! 🚀