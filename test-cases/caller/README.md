# Function `caller`

```rust
pub fn caller<E>() -> E::AccountId
where
    E: Environment,
```

## Description

The caller function retrieves the account ID of the entity that called the currently executed contract.

## Related ink! functions

- [`caller`](https://paritytech.github.io/ink/ink_env/fn.caller.html)

## Test case

The contract possesses a function named caller that allows querying the account ID of the caller. For example:

```rust
let contract = Caller::new();
let caller_id = contract.caller();
```

## Comparison Integration vs E2E

Both the integration and end-to-end tests work as expected. The default contract caller in integration tests is the Alice account, which works correctly. And in the end-to-end test, the Bob account is used to call the contract, which also returns the correct caller's account ID.

| \#  | Test                                                  | Integration | E2E |
| --- | ----------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to get the account id of the contract caller |     ✅      | ✅  |

## Result
