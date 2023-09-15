# Function `get_block_number`

## Description

The `get_block_number` function provides a mechanism to retrieve the current block number from the blockchain environment. By leveraging the ink! environment's `block_number` function, it allows smart contracts to be aware of their current block context.

```rust
pub fn block_number<E>() -> E::BlockNumber
```

and each address holds different amounts of value. There are in total 6 test default accounts ("A" to "F").

On End-to-End there's no such implementation but an [`ink_e2e::AccountKeyring`](https://paritytech.github.io/ink/ink_e2e/enum.AccountKeyring.html) `enum` type which provides the default test accounts. There are in total 8 test default accounts ("A" to "F" and two more extra).

## Related ink! functions and types

- [`block_number`](https://paritytech.github.io/ink/ink_env/fn.block_number.html)

## Test case

The primary function of the contract, `get_block_number`, retrieves the current block number. In the provided tests, this functionality is verified by advancing the block and ensuring the block number increments as expected.

| \#  | Test                                                            | Integration | E2E |
| --- | --------------------------------------------------------------- | :---------: | :-: |
| 1   | Retrieval of the current and subsequent block number.           |     ✅      | ✅  |


## Comparison Integration vs E2E

Both the integration test and the end-to-end test aim to validate the correct retrieval of the block number. The integration test advances the block within a controlled environment, while the end-to-end test interacts with the contract in a simulated real-world scenario, invoking the get_block_number method and verifying its output.

## Result

The `get_block_number` function provides essential functionality for smart contracts that need to be aware of or operate based on the current block number. This functionality is correctly implemented in bot integration and end-to-end environments.
