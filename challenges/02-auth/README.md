# Challenge 1: auth

This second challenge will depict a common bug that could have critical consequences, this challenge is about...

**Wait:exclamation: Don't you want to try solving it first?** :sunglasses:

Try finding the security vulnerability in the contract in `./src`.

If you are stuck or want to skip to the explanation of the vulnerability of this challenge, please check the [explanation page](EXPLANATION.md) or our blog post [CosmWasm Security Spotlight #1](https://medium.com/@jcsec-audits/cosmwasm-security-spotlight-1-cba294b27ea2).

## Running the tests

To run the functional tests included with this CosmWasm smart contract:
```sh
cargo test --tests -- unittests
```

## Running the proof of concept that shows the exploit

To run the proof of concept:
```sh
cargo test --tests -- exploit
```

## Audit findings

Reading and understanding real audit findings is a great way to ensure that you got a grasp of the current security topic. Please check the below list of Oak Security's audit reports :mag: where this same bug was discovered in a real-world audit:

- TBD