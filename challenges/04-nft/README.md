# Awesomwasm 2023 CTF - Challenge 08: *Gjallarhorn*

This is a clone of [Challenge 08: *Gjallarhorn*](https://github.com/oak-security/cosmwasm-ctf/tree/main/ctf-08) with the bonus solution submitted by [@baoskee](https://twitter.com/baoskee).

## Purpose

Open marketplace for an NFT project. Users can sell their own NFTs at any price or allow others to offer different NFTs in exchange to trade.

### Execute entry points:
```rust
pub enum ExecuteMsg {
    BuyNFT {
        id: String,
    },
    NewSale {
        id: String,
        price: Uint128,
        tradable: bool,
    },
    CancelSale {
        id: String,
    },
    NewTrade {
        target: String,
        offered: String,
    },
    AcceptTrade {
        id: String,
        trader: String,
    },
    CancelTrade {
        id: String,
    },
}
```

Please check the challenge's [integration_tests](./src/integration_test.rs) for expected usage examples. You can use these tests as a base to create your exploit Proof of Concept.

**:house: Base scenario:**
- The contract is newly instantiated.
- `USER1` and `USER2` placed new sales of their NFTs, one of them is open for trades and the other does not.

**:star: Goal for the challenge:**
- Demonstrate how a user can retrieve other users' NFT for free.

:exclamation: The usage of [`cw-multi-test`](https://github.com/CosmWasm/cw-multi-test) is **mandatory** for the PoC, please take the approach of the provided integration tests as a suggestion.

:exclamation: Remember that insider threats and centralization concerns are out of the scope of the CTF.
