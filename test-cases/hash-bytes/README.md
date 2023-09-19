# Function `hash_bytes`

```rust
pub fn hash_bytes<H>(input: &[u8], output: &mut <H as HashOutput>::Type)
```

## Description

The `hash_bytes` module provides functionality to compute the hash of a given byte array using the designated hash function. The method `get_hash_bytes` accepts an array of bytes and returns its hash. This function is essential for cryptographic operations where data integrity and verification are crucial.

## Related ink! functions

- [`hash_bytes`](https://paritytech.github.io/ink/ink_env/fn.hash_bytes.html)

## Test case

Both test cases instantiate the `HashBytes` contract and attempt to compute the hash of a given byte array. The tests then validate the result based on the expected hash length, ensuring the correctness of the hash computation.

## Comparison Integration vs E2E

The function `get_hash_bytes` has been successfully validated in both integration and end-to-end test scenarios, demonstrating its robustness and consistent implementation.

| \#  | Test                                              | Integration | E2E |
| --- | ------------------------------------------------- | :---------: | :-: |
| 1   | Attempts to retrieve the code hash given an input |     ✅      | ✅  |

## Result

- This functionality is implemented and works as expected
