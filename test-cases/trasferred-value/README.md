# Function `name_function`

```rust
pub fn transferred_value<E>() -> E::Balance
where
    E: Environment,
```

## Description

Returns the transferred value for the contract execution.

## Related ink! functions

- [transferred_value](https://paritytech.github.io/ink/ink_env/fn.transferred_value.html)

## Test case

The test sends 1000 tokens to the contract through the function `send_tokens_and_get_transferred_message`. The function is `payable` so it is possible to send tokens in the call. Finally it returns the amount of tokens sent by querying the `transferred_value` function.

| \#  | Test            | Integration | E2E |
| --- | --------------- | :---------: | :-: |
| 1   | Attempts to send token amount and get the amount sent through the function `transferred value`. |     ✅      | ✅  |

## Comparison Integration vs E2E

The function works in both e2e and integration test.

## Result

- Can it be implemented?
- How long will it take?
- Ideas for implementation
- Other notes
