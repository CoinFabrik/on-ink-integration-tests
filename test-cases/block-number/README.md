# Function `block_number`

```rust
pub fn block_number<E>() -> E::BlockNumber
```

## Description

The `get_block_number` function provides a mechanism to retrieve the current block number from the blockchain environment. By leveraging the ink! environment's `block_number` function, it allows smart contracts to be aware of their current block context.


## Related ink! functions and types

- [`block_number`](https://paritytech.github.io/ink/ink_env/fn.block_number.html)

## Test case

The primary function of the contract, `get_block_number`, retrieves the current block number. In the provided integration tests, this functionality is verified by advancing the block and ensuring the block number increments as expected. In the end-to-end tests, the block number increments upon contract instantiation and further contract calls.

## Comparison Integration vs E2E

In the integration test environment, simply advancing the block increments the block number. For instance, when the block is advanced, the block number changes from `0` to `1`.

However, in the end-to-end test environment, there's a noticeable difference. Instantiating the contract and making subsequent calls to it creates new blocks, resulting in an incremented block number for each call. Specifically, after instantiating the contract, the block number is `1`. A subsequent call to retrieve the block number results in a block number of `2`.

| \#  | Test                                                            | Integration | E2E |
| --- | --------------------------------------------------------------- | :---------: | :-: |
| 1   | Retrieval of the current and subsequent block number.           |     ✅      | ✅  |

## Result

The `block_number` function provides essential functionality for smart contracts that need to be aware of or operate based on the current block number. This functionality is correctly implemented in both integration and end-to-end environments.
