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

The integration tests allow for setting and verifying the contract's balance using the set_account_balance function.

In the e2e tests, the ink_e2e crate is utilized to perform end-to-end testing. The test initializes a contract instance with a specific balance. It then verifies the get_balance function by calling the contract and checking if the returned balance is as expected, taking into account any balance added during the call.

| \#  | Test                                                    | Integration | E2E |
| --- | ------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the balance of the executing contract   |     ✅      | ✅  |

## Result

This functionality is implemented and work as expected.
