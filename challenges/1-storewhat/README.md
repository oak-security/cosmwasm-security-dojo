# Challenge 1: Store-what?

This first challenge will depict a common bug that could have critical consequences, this challenge is about... 

**Wait:exclamation: Don't you want to try it first?** :sunglasses:

If you ready to learn more about the security issue depicted in this first challeng, please check  [CosmWasm Security Spotligth #1](notion://www.notion.so/Dojo-1d3e575f21de4173a28cade2e67d1796)

## Running the tests

To run the functional tests included with this CosmWasm smart contract:
```sh
cargo test --tests -- unittests --nocapture
```

To run the proof of concept:
```sh
cargo test --tests -- exploit  --nocapture
```

## Audit findings

Reading and understanding real audit findings is a great way to ensure that you got a grasp of the current security topic. Please check the below list of :mag:Oak's audit reports:mag: where this same bug was discovered in a real professional audit!

- [Report 1, finding #2 "LOCKINGADDRESS is never populated which will block rebasing functionality"](https://github.com/oak-security/audit-reports/blob/master/Comdex/2022-10-28%20Audit%20Report%20-%20Comdex%20Locking%20and%20Vesting%20Contracts%20v1.0.pdf)
- [Report 2, finding #1 "Bad debt state is not recorded"](https://github.com/oak-security/audit-reports/blob/master/Margined%20Protocol/2022-10-28%20Audit%20Report%20-%20Margined%20Protocol%20Perpetuals%20v1.0.pdf)
- [Report 3, finding #2 of "State update not stored"](https://github.com/oak-security/audit-reports/blob/master/Prism/2022-11-04%20Audit%20Report%20-%20Prism%20Auto%20Compounding%20cAsset%20v1.0.pdf)
