# Function `hash_encoded`

```rust
pub fn hash_encoded<H, T>(input: &T, output: &mut <H as HashOutput>::Type)
```

## Description

The `hash_encoded` module offers the capability to first encode the given input using SCALE encoding and then compute its cryptographic hash using the specified hash function. The `hash_encoded` function is integral for operations where encoded data needs to be verified or its integrity ensured through hashing.

## Related ink! functions

- [`hash_encoded`](https://paritytech.github.io/ink/ink_env/fn.hash_encoded.html)

## Test case

Both the integration tests and end-to-end tests in the `hash_encoded` module verify the hashing and encoding functionality of the contract. Each test ensures that the corresponding hash function produces the expected hash output for a given input.

## Comparison Integration vs E2E

The function `hash_encoded` has been successfully validated in both integration and end-to-end test scenarios, demonstrating its robustness and consistent implementation.

| #   | Test                                                   | Integration | E2E |
| --- | ------------------------------------------------------ | :---------: | :-: |
| 1   | Attempts to get the hash using the method `sha_256`    |     ✅      | ✅  |
| 2   | Attempts to get the hash using the method `blake_128`  |     ✅      | ✅  |
| 3   | Attempts to get the hash using the method `blake_256`  |     ✅      | ✅  |
| 4   | Attempts to get the hash using the method `keccak_256` |     ✅      | ✅  |

## Result

- This functionality is implemented and works as expected
