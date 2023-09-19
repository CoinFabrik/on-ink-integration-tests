# Function `balance`

```rust
pub fn balance<E>() -> E::Balance
```

## Description

The `get_balance` function retrieves the balance of the contract.

## Related ink! functions

- [`balance`](https://paritytech.github.io/ink/ink_env/fn.balance.html)

## Test case

The contract possesses a function named get_balance that allows querying the balance of the contract. For example:

```rust
let contract = GetBalance::new();
let balance = contract.get_balance();
```

## Comparison Integration vs E2E

The integration tests allow for setting and verifying the contract's balance using the `set_account_balance` function. In these tests, the balance is set directly to the specified amount. However, in the end-to-end tests, despite initializing the contract with a balance of `50`, there's an observed additional default balance. As a result, the contract starts with an unexpected total initial balance of `1000000050`. The reason for this default balance is currently unknown.


| \#  | Test                                                    | Integration | E2E |
| --- | ------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the balance of the executing contract   |     ✅      | ✅  |

## Result

This functionality is implemented and work as expected.
