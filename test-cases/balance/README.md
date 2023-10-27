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

## Result

We need to understand where the initial balance value is created/used in the _integration test environment_ and update 
its value to the value in the _e2e environment_.
