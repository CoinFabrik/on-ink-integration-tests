# Function `balance`

```rust
pub fn balance<E>() -> E::Balance
```

## Description

The `balance` function retrieves the balance of the contract.

## Related ink! functions

- [`balance`](https://paritytech.github.io/ink/ink_env/fn.balance.html)

## Test case

We create a contract in a defined address in order to retrieve the account initial balance and compare the results 
across the environments.

## Comparison Integration vs E2E

We found that the initial balance differs on both environments.

| \#  | Test                  | Integration |     E2E      | OK |
| --- |-----------------------|:-----------:|:------------:|:--:|
| 1   | `get_balance`         |   1000000   | 1000000000   | ❌  |

Furthermore, the inital balance of default addresses also differed in both environments. 

| \#  | Test                  | Integration |     E2E      | OK |
| --- |-----------------------|:-----------:|:------------:|:--:|
| 2   | `default_balances`         |   1000000   | 1<<60   | ❌  |

## Result

We need to understand where the initial balance value is created/used in the _integration test environment_ and update 
its value to the value in the _e2e environment_. 

We also need to modify the inital balance of default addresses in integration tests to equal e2e tests.

## Update on Correcting this Issue

In [PR #1982](https://github.com/paritytech/ink/pull/1982) we updated the default balance in integration tests to be the same as in e2e-tests. This is not definitive, as we want opinions if this should be like this. 

We also corrected the initial balance of default addresses, updating [PR #1955](https://github.com/paritytech/ink/pull/1955) from Milestone 2.
