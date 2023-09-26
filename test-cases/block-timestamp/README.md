# Function `block_timestamp`

```rust
pub fn block_timestamp<E>() -> E::Timestamp
where
    E: Environment,
```

## Description

`block_timestamp` returns the current block timestamp.

## Related ink! functions

- [block_timestamp](https://paritytech.github.io/ink/ink_env/fn.block_timestamp.html)
- [advance_block](https://paritytech.github.io/ink/ink_env/test/fn.advance_block.html)

## Test case

The test asks the contract for the block_timestamp at two different times and then compares them to verify that the first is less than the second.

| \#  | Test            | Integration | E2E |
| --- | --------------- | :---------: | :-: |
| 1   | Attempts to check that the timestamp is advancing |     ✅      | ✅  |

## Comparison Integration vs E2E

The `block_timestamp` function works in both cases. However, in integration test you have to advance the block manually to get a different value.

## Result

This functionality is implemented and works as expected.