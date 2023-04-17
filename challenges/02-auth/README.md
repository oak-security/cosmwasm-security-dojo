# Challenge 2: auth

This second challenge will depict a common bug that could have critical consequences, this challenge is about...

**Wait:exclamation: Don't you want to try solving it first?** :sunglasses:

Try finding the security vulnerability in the contract in `./src`.

If you are stuck or want to skip to the explanation of the vulnerability of this challenge, please check the [explanation page](EXPLANATION.md) or our blog post [CosmWasm Security Spotlight #2](https://jcsec-audits.medium.com/cosmwasm-security-spotlight-2-3b8abeb066a1).

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

- [Report 1 finding #1 "Task contract execute_update_config is permissionless"](https://github.com/oak-security/audit-reports/blob/master/CronCat/2023-03-14%20Audit%20Report%20-%20CronCat%20CosmWasm%20v1.0.pdf)
- [Report 2 finding #1 "Incorrect permissioning of IbcExecuteProposal execution leads to failure of proposal execution and elevated owner privileges"](https://github.com/oak-security/audit-reports/blob/master/Astroport/2023-02-14%20Audit%20Report%20-%20Astroport%20IBC%20v1.0.pdf)
- [Report 3 finding #6 "Emergency ShutDownVamms messages are not able to execute SetOpen transactions due to lack of permissions"](https://github.com/oak-security/audit-reports/blob/master/Margined%20Protocol/2022-10-28%20Audit%20Report%20-%20Margined%20Protocol%20Perpetuals%20v1.0.pdf)
