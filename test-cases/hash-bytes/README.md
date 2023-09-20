# Function `hash_bytes`

```rust
pub fn hash_bytes<H>(input: &[u8], output: &mut <H as HashOutput>::Type)
```

## Description

The `hash_bytes` module provides functionality to compute the hash of a given byte array using the designated hash function. The method `get_hash_bytes` accepts an array of bytes and returns its hash. This function is essential for cryptographic operations where data integrity and verification are crucial.

## Related ink! functions

- [`hash_bytes`](https://paritytech.github.io/ink/ink_env/fn.hash_bytes.html)

## Test case

Both the integration tests and end-to-end tests in the `hash_bytes` module verify the hashing functionality of the contract. Each test ensures that the corresponding hash function produces the expected hash output for a given input.

## Comparison Integration vs E2E

The function `get_hash_bytes` has been successfully validated in both integration and end-to-end test scenarios, demonstrating its robustness and consistent implementation.

| #   | Test                                                   | Integration | E2E |
| --- | ------------------------------------------------------ | :---------: | :-: |
| 1   | Attempts to get the hash using the method `sha_256`    |     ✅      | ✅  |
| 2   | Attempts to get the hash using the method `blake_128`  |     ✅      | ✅  |
| 3   | Attempts to get the hash using the method `blake_256`  |     ✅      | ✅  |
| 4   | Attempts to get the hash using the method `keccak_256` |     ✅      | ✅  |

## Result

- This functionality is implemented and works as expected
