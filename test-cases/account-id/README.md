# Function `account_id`

```rust
pub fn account_id<E>() -> E::AccountId
```

## Description

The `account_id` module provides functionality to retrieve the account ID of the currently executed contract.

## Related ink! functions

- List of functions called in the ink repository

## Test case

Both the integration and end-to-end test's work as expected. The integration test sets the callee address, and then retrieves it with the `account_id` function. While in the end-to-end test the account id is the result of the contract instantiation, and this is then compared to the output of the function.

## Comparison Integration vs E2E

| \#  | Test                                                          | Integration | E2E |
| --- | ------------------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to retrieve the account id of the executing contract |     ✅      | ✅  |

## Result

This functionality is implemented and works correctly
