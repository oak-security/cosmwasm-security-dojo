# Challenge 5: Addressing

This fifth challenge will depict a common bug that could have critical consequences, this challenge is about...

**Wait:exclamation: Don't you want to try solving it first?** :sunglasses:

Try finding the security vulnerability in the contract in `./src`.

If you are stuck or want to skip to the explanation of the vulnerability of this challenge, please check the [explanation page](EXPLANATION.md) or our blog post [CosmWasm Security Spotlight #3](https://medium.com/@jcsec-audits/cosmwasm-security-spotlight-3-2b11f36fd61).//@todo

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

- [Report 1, Finding #1 "Attacker can bypass self-call validation."](https://github.com/oak-security/audit-reports/blob/master/CronCat/2023-03-14%20Audit%20Report%20-%20CronCat%20CosmWasm%20v1.0.pdf)
- [Report 2, Finding #5 "Lack of address validation could lead to locked funds."](https://github.com/oak-security/audit-reports/blob/master/Astroport/2023-02-10%20Audit%20Report%20-%20Astroport%20Core%20Updates%20v1.0.pdf)
- [Report 3, Finding #13 "Lack of address validation might cause errors when using invalid stored addresses"](https://github.com/oak-security/audit-reports/blob/master/Astroport/2022-01-20%20Audit%20Report%20-%20Astroport%20v1.0.pdf)
