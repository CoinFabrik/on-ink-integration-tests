# Function `minimum_balance`

```rust
pub fn minimum_balance<E>() -> E::Balance
where
    E: Environment,
```

## Description

`minimum_balance` Returns the minimum balance that is required for creating an account (i.e. the chain’s existential deposit).

## Related ink! functions

- [minimum_balance](https://paritytech.github.io/ink/ink_env/fn.minimum_balance.html#)

## Test case

How the test cases actually tests the given functionality.

| \#  | Test            | Integration | E2E |
| --- | --------------- | :---------: | :-: |
| 1   | Attempts to obtaining the minimum balance |     ✅      | ✅  |

## Comparison Integration vs E2E

The function works in both environments correctly.

## Result

This functionality is implemented and works as expected.
