# Challenge 1: Store-what?

This first challenge will depict a common bug that could have critical consequences, this challenge is about...

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

- [Report 1, finding #2 "LOCKINGADDRESS is never populated which will block rebasing functionality"](https://github.com/oak-security/audit-reports/blob/master/Comdex/2022-10-28%20Audit%20Report%20-%20Comdex%20Locking%20and%20Vesting%20Contracts%20v1.0.pdf)
- [Report 2, finding #1 "Bad debt state is not recorded"](https://github.com/oak-security/audit-reports/blob/master/Margined%20Protocol/2022-10-28%20Audit%20Report%20-%20Margined%20Protocol%20Perpetuals%20v1.0.pdf)
- [Report 3, finding #2 "State update not stored"](https://github.com/oak-security/audit-reports/blob/master/Prism/2022-11-04%20Audit%20Report%20-%20Prism%20Auto%20Compounding%20cAsset%20v1.0.pdf)
