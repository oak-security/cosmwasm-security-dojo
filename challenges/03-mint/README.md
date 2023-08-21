# Awesomwasm 2023 CTF - Challenge 10

This is a clone of [Challenge 10: *Mistilteinn*](https://github.com/oak-security/cosmwasm-ctf/tree/main/ctf-10) with the bonus solution submitted by [@CruncherDefi](https://twitter.com/CruncherDefi) and [@howlpack](https://twitter.com/howlpack).

## Purpose

This smart contract allows whitelisted users to mint NFTs. 

### Execute entry points:
```rust
pub enum ExecuteMsg {
    Mint {},
}
```

Please check the challenge's [integration_tests](./src/integration_tests.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is instantiated with whitelisted users as `USER1`, `USER2`, and `USER3`.

**:star: Goal for the challenge:**
- Demonstrate how a misconfiguration from the contract instantiator allows users to bypass the `mint_per_user` limitation.

:exclamation: The usage of [`cw-multi-test`](https://github.com/CosmWasm/cw-multi-test) is **mandatory** for the PoC, please take the approach of the provided integration tests as a suggestion.
