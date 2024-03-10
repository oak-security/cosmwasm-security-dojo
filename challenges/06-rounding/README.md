# Challenge 6: Rounding

This sixth challenge will depict a common bug that could have critical consequences, this challenge is about...

**Wait :exclamation: Don't you want to try solving it first?** :sunglasses:

Try finding the security vulnerability in the contract in `./src`.

If you are stuck or want to skip to the explanation of the vulnerability of this challenge, please check the [explanation page](EXPLANATION.md) or our blog post [CosmWasm Security Spotlight #4](https://medium.com/@jcsec-audits/cosmwasm-security-spotlight-4-b5ba69b96c5f).

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

- [Report 1, finding #4 "Possible inconsistencies when configuring decimal values."](https://github.com/oak-security/audit-reports/blob/master/Margined%20Protocol/2022-10-28%20Audit%20Report%20-%20Margined%20Protocol%20Perpetuals%20v1.0.pdf)
- [Report 2, finding #1 "Attackers can cause a consensus failure by sending coins through IBC"](https://github.com/oak-security/audit-reports/blob/master/Noble/2023-07-08%20Audit%20Report%20-%20Noble%20Tariff%20Module%20v1.0.pdf)
- [Report 3, finding #3 "Multiple rounding issues may cause zero rewards to be distributed"](https://github.com/oak-security/audit-reports/blob/master/Comdex/2022-10-28%20Audit%20Report%20-%20Comdex%20Locking%20and%20Vesting%20Contracts%20v1.0.pdf)
